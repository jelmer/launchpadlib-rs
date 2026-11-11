#![warn(missing_docs)]
//! # Launchpad API
//!
//! This crate provides a Rust interface to the Launchpad API.
//! It is generated from the Launchpad API WADL document.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use url::Url;
//!
//! #[cfg(all(feature = "api-v1_0", feature = "blocking"))]
//! {
//! let client = launchpadlib::blocking::Client::anonymous("just+testing");
//! let service_root = launchpadlib::blocking::v1_0::service_root(&client).unwrap();
//! let people = service_root.people().unwrap();
//! let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
//! let ssh_keys = person.sshkeys(&client).unwrap().map(|k| k.unwrap().keytext).collect::<Vec<_>>();
//! println!("SSH Keys: {:?}", ssh_keys);
//! }
//! ```
//!
//! ## Limitations and bugs
//!
//! * While bindings are generated from the entire WADL file, I have only used a small number of
//!   them. Please report bugs if you run into issues.  Launchpad's WADL is incorrect in places, e.g.
//!   claiming that certain fields are optional while they will actually be set to null. Any problems
//!   with the WADL will impact the usability of the rust bindings.
//!
//! * See fixup.xsl for manual patches that are applied; this file is almost certainly incomplete.

pub mod auth;
pub mod uris;
pub use wadl::{Error, Resource};

/// The default user agent, used if none is provided
pub const DEFAULT_USER_AGENT: &str =
    concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// The default Launchpad instance
pub const DEFAULT_INSTANCE: &str = "launchpad.net";

#[cfg(feature = "async")]
pub mod r#async;

#[cfg(feature = "blocking")]
pub mod blocking;

#[allow(dead_code)]
pub(crate) trait AsTotalSize {
    fn into_total_size(self) -> Option<usize>;
}

impl AsTotalSize for Option<usize> {
    fn into_total_size(self) -> Option<usize> {
        self
    }
}

impl AsTotalSize for usize {
    fn into_total_size(self) -> Option<usize> {
        Some(self)
    }
}

/// Various custom types to help massaging the LP data into proper Rust types.
pub mod types {
    /// Custom type to work around some peculiarities of the package_upload.display_arches field.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub enum PackageUploadArches {
        /// A sourceful upload
        #[serde(rename = "source")]
        Source,
        /// When the upload comes from a Debian sync, there is no arch list.
        #[serde(rename = "sync")]
        Sync,
        /// A single arch
        #[serde(untagged)]
        Arch(String),
        /// Several arches for a single item. Obsolete?
        #[serde(untagged)]
        Arches(Vec<String>),
    }

    /// A generic wrapper type for fields that may be redacted for private projects.
    /// Some fields in Launchpad can return "tag:launchpad.net:2008:redacted"
    /// instead of their actual value for private projects.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum MaybeRedacted<T> {
        /// The actual value
        Value(T),
        /// A redacted value for private projects
        Redacted,
    }

    impl<T> MaybeRedacted<T> {
        /// Get the inner value as an Option, returning None if redacted
        pub fn as_option(&self) -> Option<&T> {
            match self {
                Self::Value(v) => Some(v),
                Self::Redacted => None,
            }
        }

        /// Get the inner value as an Option, consuming self
        pub fn into_option(self) -> Option<T> {
            match self {
                Self::Value(v) => Some(v),
                Self::Redacted => None,
            }
        }

        /// Check if the value is redacted
        pub fn is_redacted(&self) -> bool {
            matches!(self, Self::Redacted)
        }

        /// Get the inner value or a default if redacted
        pub fn unwrap_or(self, default: T) -> T {
            match self {
                Self::Value(v) => v,
                Self::Redacted => default,
            }
        }

        /// Get the inner value or compute it from a closure if redacted
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T,
        {
            match self {
                Self::Value(v) => v,
                Self::Redacted => f(),
            }
        }

        /// Map the inner value if present
        pub fn map<U, F>(self, f: F) -> MaybeRedacted<U>
        where
            F: FnOnce(T) -> U,
        {
            match self {
                Self::Value(v) => MaybeRedacted::Value(f(v)),
                Self::Redacted => MaybeRedacted::Redacted,
            }
        }
    }

    impl<T> Default for MaybeRedacted<T>
    where
        T: Default,
    {
        fn default() -> Self {
            Self::Value(T::default())
        }
    }

    impl<T> From<T> for MaybeRedacted<T> {
        fn from(value: T) -> Self {
            Self::Value(value)
        }
    }

    impl<T> serde::Serialize for MaybeRedacted<T>
    where
        T: serde::Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Self::Value(v) => v.serialize(serializer),
                Self::Redacted => "tag:launchpad.net:2008:redacted".serialize(serializer),
            }
        }
    }

    impl<'de, T> serde::Deserialize<'de> for MaybeRedacted<T>
    where
        T: serde::Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(serde::Deserialize)]
            #[serde(untagged)]
            enum MaybeRedactedHelper<T> {
                Value(T),
                String(String),
            }

            match MaybeRedactedHelper::<T>::deserialize(deserializer)? {
                MaybeRedactedHelper::Value(v) => Ok(Self::Value(v)),
                MaybeRedactedHelper::String(s) => {
                    if s == "tag:launchpad.net:2008:redacted" {
                        Ok(Self::Redacted)
                    } else {
                        Err(serde::de::Error::custom(format!(
                            "expected value or redacted tag, got string: {}",
                            s
                        )))
                    }
                }
            }
        }
    }

    /// Implement Display as JSON, which is how Launchpad expects structured
    /// values to be passed as method parameters.
    macro_rules! display_as_json {
        ($t:ty) => {
            impl std::fmt::Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let s = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
                    f.write_str(&s)
                }
            }
        };
    }

    macro_rules! json_map {
        ($(#[$meta:meta])* $name:ident, $k:ty, $v:ty) => {
            $(#[$meta])*
            #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
            #[serde(transparent)]
            pub struct $name(pub std::collections::BTreeMap<$k, $v>);

            impl std::ops::Deref for $name {
                type Target = std::collections::BTreeMap<$k, $v>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl From<std::collections::BTreeMap<$k, $v>> for $name {
                fn from(map: std::collections::BTreeMap<$k, $v>) -> Self {
                    Self(map)
                }
            }

            display_as_json!($name);
        };
    }

    json_map!(
        /// Mapping from snap names (e.g. "core22", "snapcraft") to the channels to
        /// install them from.
        SnapChannels,
        String,
        String
    );

    /// Snap channels used for builds against a base.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct BaseChannels {
        /// Mapping from snap names to channels.
        #[serde(flatten)]
        pub channels: std::collections::BTreeMap<String, String>,

        /// Per-architecture overrides of `channels`.
        #[serde(
            rename = "_byarch",
            default,
            skip_serializing_if = "std::collections::BTreeMap::is_empty"
        )]
        pub by_arch: std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    }

    display_as_json!(BaseChannels);

    /// A feature that a snap base can support.
    #[derive(
        Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
    )]
    pub enum SnapBaseFeature {
        /// Allow items with duplicate build-on attribute in the list of architectures.
        #[serde(rename = "allow_duplicate_build_on")]
        AllowDuplicateBuildOn,
    }

    json_map!(
        /// Features supported by a snap base.
        SnapBaseFeatures,
        SnapBaseFeature,
        bool
    );

    json_map!(
        /// ARG variables to pass to an OCI image build.
        BuildArgs,
        String,
        String
    );

    json_map!(
        /// Free-form metadata about a live filesystem image, passed to the builder.
        LiveFSMetadata,
        String,
        serde_json::Value
    );

    json_map!(
        /// Free-form metadata for artifacts attached to a revision status report.
        RevisionStatusProperties,
        String,
        serde_json::Value
    );

    /// Overrides for the metadata in an archive's Release files.
    ///
    /// Values may use a `{series}` placeholder.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct ArchiveMetadataOverrides {
        /// Value for the Origin field.
        #[serde(rename = "Origin", default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,

        /// Value for the Label field.
        #[serde(rename = "Label", default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,

        /// Value for the Suite field.
        #[serde(rename = "Suite", default, skip_serializing_if = "Option::is_none")]
        pub suite: Option<String>,

        /// Value for the Snapshots field.
        #[serde(rename = "Snapshots", default, skip_serializing_if = "Option::is_none")]
        pub snapshots: Option<String>,
    }

    display_as_json!(ArchiveMetadataOverrides);

    /// Identity of a social account. Which fields are set depends on the platform.
    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct SocialAccountIdentity {
        /// Username on the platform.
        pub username: String,

        /// Matrix homeserver, for Matrix accounts.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub homeserver: Option<String>,
    }

    display_as_json!(SocialAccountIdentity);

    /// An error from the last attempt to upload a snap build to the store.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct StoreUploadErrorMessage {
        /// Description of the error.
        pub message: String,

        /// External link with more details about the error.
        #[serde(default)]
        pub link: Option<url::Url>,
    }

    /// An error returned by an OCI registry, as described in
    /// <https://docs.docker.com/registry/spec/api/#errors>.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct RegistryError {
        /// Error code, e.g. "BLOB_UNKNOWN".
        pub code: String,

        /// Human readable description of the error.
        pub message: String,

        /// Unstructured details about the error; the format depends on the code.
        #[serde(default)]
        pub detail: Option<serde_json::Value>,
    }

    /// Type of a landmark in a project series timeline.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum TimelineLandmarkType {
        /// A milestone.
        Milestone,
        /// A release.
        Release,
    }

    /// A milestone or release in a project series timeline.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct TimelineLandmark {
        /// Name of the milestone.
        pub name: String,

        /// Code name of the milestone.
        pub code_name: Option<String>,

        /// Whether this is a milestone or a release.
        #[serde(rename = "type")]
        pub landmark_type: TimelineLandmarkType,

        /// Expected date for milestones, release date for releases.
        pub date: Option<chrono::NaiveDate>,

        /// Path of the milestone or release.
        pub uri: String,
    }

    /// A job in a CI build pipeline stage.
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct CIJob(
        /// Name of the job.
        pub String,
        /// Index of the job among the jobs with the same name.
        pub usize,
    );

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::BTreeMap;

        #[test]
        fn test_base_channels() {
            let json =
                r#"{"core22": "latest/stable", "_byarch": {"riscv64": {"core22": "latest/edge"}}}"#;
            let channels: BaseChannels = serde_json::from_str(json).unwrap();
            assert_eq!(
                channels,
                BaseChannels {
                    channels: BTreeMap::from([("core22".into(), "latest/stable".into())]),
                    by_arch: BTreeMap::from([(
                        "riscv64".into(),
                        BTreeMap::from([("core22".into(), "latest/edge".into())])
                    )]),
                }
            );
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&channels.to_string()).unwrap(),
                serde_json::from_str::<serde_json::Value>(json).unwrap()
            );
        }

        #[test]
        fn test_snap_channels_display() {
            let channels = SnapChannels::from(BTreeMap::from([(
                "snapcraft".to_string(),
                "8.x/stable".to_string(),
            )]));
            assert_eq!(channels.to_string(), r#"{"snapcraft":"8.x/stable"}"#);
        }

        #[test]
        fn test_snap_base_features() {
            let features: SnapBaseFeatures =
                serde_json::from_str(r#"{"allow_duplicate_build_on": true}"#).unwrap();
            assert_eq!(
                features.get(&SnapBaseFeature::AllowDuplicateBuildOn),
                Some(&true)
            );
        }

        #[test]
        fn test_archive_metadata_overrides() {
            let overrides: ArchiveMetadataOverrides =
                serde_json::from_str(r#"{"Origin": "LP-PPA-foo", "Suite": "{series}"}"#).unwrap();
            assert_eq!(
                overrides,
                ArchiveMetadataOverrides {
                    origin: Some("LP-PPA-foo".into()),
                    label: None,
                    suite: Some("{series}".into()),
                    snapshots: None,
                }
            );
            assert_eq!(
                overrides.to_string(),
                r#"{"Origin":"LP-PPA-foo","Suite":"{series}"}"#
            );
        }

        #[test]
        fn test_timeline_landmark() {
            let landmark: TimelineLandmark = serde_json::from_str(
                r#"{"name": "1.0", "code_name": null, "type": "release", "date": "2024-01-02", "uri": "/foo/+milestone/1.0"}"#,
            )
            .unwrap();
            assert_eq!(
                landmark,
                TimelineLandmark {
                    name: "1.0".into(),
                    code_name: None,
                    landmark_type: TimelineLandmarkType::Release,
                    date: chrono::NaiveDate::from_ymd_opt(2024, 1, 2),
                    uri: "/foo/+milestone/1.0".into(),
                }
            );
        }

        #[test]
        fn test_ci_stages() {
            let stages: Vec<Vec<CIJob>> =
                serde_json::from_str(r#"[[["build", 0]], [["test", 0], ["test", 1]]]"#).unwrap();
            assert_eq!(
                stages,
                vec![
                    vec![CIJob("build".into(), 0)],
                    vec![CIJob("test".into(), 0), CIJob("test".into(), 1)],
                ]
            );
        }
    }
}

#[cfg(all(feature = "api-devel", feature = "blocking"))]
pub mod devel {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    use super::*;
    use crate::AsTotalSize;
    use url::Url;
    include!(concat!(env!("OUT_DIR"), "/generated/devel.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/devel/").unwrap());
    }

    /// Get the default service root
    pub fn service_root(
        client: &dyn wadl::blocking::Client,
    ) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust,no_run
    /// let client = launchpadlib::Client::anonymous("just+testing");
    /// let root = launchpadlib::devel::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::blocking::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, Error> {
        let url = Url::parse(&format!("https://{}/devel/", host)).unwrap();
        ServiceRoot(url).get(client)
    }
}

#[cfg(all(feature = "api-beta", feature = "blocking"))]
pub mod beta {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    use super::*;
    use crate::AsTotalSize;
    use url::Url;
    include!(concat!(env!("OUT_DIR"), "/generated/beta.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/beta/").unwrap());
    }

    /// Get the default service root
    pub fn service_root(
        client: &dyn wadl::blocking::Client,
    ) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust,no_run
    /// let client = launchpadlib::Client::anonymous("just+testing");
    /// let root = launchpadlib::beta::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::blocking::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, Error> {
        let url = Url::parse(&format!("https://{}/beta/", host)).unwrap();
        ServiceRoot(url).get(client)
    }
}

#[cfg(all(feature = "api-v1_0", feature = "blocking"))]
/// Version 1.0 of the Launchpad API
pub mod v1_0 {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    use super::*;
    use crate::AsTotalSize;
    use url::Url;

    include!(concat!(env!("OUT_DIR"), "/generated/1_0.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/1.0/").unwrap());
        static ref STATIC_RESOURCES: std::collections::HashMap<Url, Box<dyn Resource + Send + Sync>> = {
            let mut m = std::collections::HashMap::new();
            let root = ServiceRoot(Url::parse("https://api.launchpad.net/1.0/").unwrap());
            m.insert(root.url().clone(), Box::new(root) as Box<dyn Resource + Send + Sync>);
            m
        };
    }

    /// Get the service root by URL
    pub fn get_service_root_by_url(
        url: &'_ Url,
    ) -> std::result::Result<&'static ServiceRoot, Error> {
        if url == ROOT.url() {
            Ok(&ROOT)
        } else {
            Err(Error::InvalidUrl)
        }
    }

    /// Get the default service root
    pub fn service_root(
        client: &dyn wadl::blocking::Client,
    ) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust,no_run
    /// let client = launchpadlib::blocking::Client::anonymous("just+testing");
    /// let root = launchpadlib::v1_0::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::blocking::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, Error> {
        let url = Url::parse(&format!("https://{}/1.0/", host)).unwrap();
        ServiceRoot(url).get(client)
    }

    /// Get a resource by its URL
    pub fn get_resource_by_url(
        url: &'_ Url,
    ) -> std::result::Result<&'static (dyn Resource + Send + Sync), Error> {
        if let Some(resource) = STATIC_RESOURCES.get(url) {
            Ok(resource.as_ref())
        } else {
            Err(Error::InvalidUrl)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_person() {
            let json = include_str!("../testdata/person.json");
            let person: PersonFull = serde_json::from_str(json).unwrap();
            assert_eq!(person.display_name, "Jelmer Vernooij");
            assert_eq!(person.id, crate::types::MaybeRedacted::Redacted);
        }

        #[test]
        fn test_parse_person_nullable_fields() {
            let mut json: serde_json::Value =
                serde_json::from_str(include_str!("../testdata/person.json")).unwrap();
            for field in [
                "archive_link",
                "description",
                "preferred_email_address_link",
            ] {
                json[field] = serde_json::Value::Null;
            }
            let person: PersonFull = serde_json::from_value(json).unwrap();
            #[cfg(feature = "packages")]
            assert_eq!(person.archive_link, None);
            assert_eq!(person.description, None);
            assert_eq!(person.preferred_email_address_link, None);
        }

        #[test]
        fn test_parse_team() {
            let json = include_str!("../testdata/team.json");
            let team: TeamFull = serde_json::from_str(json).unwrap();
            assert_eq!(team.display_name, "awsome-core");

            let json = include_str!("../testdata/team2.json");
            let _team: TeamFull = serde_json::from_str(json).unwrap();
        }

        #[test]
        #[cfg(feature = "bugs")]
        fn test_parse_bug() {
            let json = include_str!("../testdata/bug.json");
            let bug: BugFull = serde_json::from_str(json).unwrap();
            assert_eq!(bug.title, "Microsoft has a majority market share");

            let json = include_str!("../testdata/bug2.json");
            let bug: BugFull = serde_json::from_str(json).unwrap();
            assert_eq!(bug.name, None);
            assert_eq!(bug.id, 2039729);
        }

        #[test]
        #[cfg(feature = "bugs")]
        fn test_parse_bug_without_last_message() {
            let mut json: serde_json::Value =
                serde_json::from_str(include_str!("../testdata/bug.json")).unwrap();
            json["date_last_message"] = serde_json::Value::Null;
            let bug: BugFull = serde_json::from_value(json).unwrap();
            assert_eq!(bug.date_last_message, None);
        }

        #[test]
        #[cfg(feature = "bugs")]
        fn test_parse_bug_tasks() {
            let json = include_str!("../testdata/bug_tasks.json");
            let _bug_tasks: BugTaskPage = serde_json::from_str(json).unwrap();
        }
    }

    #[cfg(feature = "bugs")]
    impl Bugs {
        /// Get a bug by its id
        ///
        /// # Example
        /// ```rust,no_run
        /// let client = launchpadlib::blocking::Client::anonymous("just+testing");
        /// let root = launchpadlib::v1_0::service_root(&client).unwrap();
        /// let bug = root.bugs().unwrap().get_by_id(&client, 1).unwrap();
        /// ```
        pub fn get_by_id(
            &self,
            client: &dyn wadl::blocking::Client,
            id: u32,
        ) -> std::result::Result<BugFull, Error> {
            let url = self.url().join(format!("bugs/{}", id).as_str()).unwrap();
            Bug(url).get(client)
        }
    }

    impl Projects {
        /// Get a project by name
        pub fn get_by_name(
            &self,
            client: &dyn wadl::blocking::Client,
            name: &str,
        ) -> std::result::Result<ProjectFull, Error> {
            let url = self.url().join(name).unwrap();
            Project(url).get(client)
        }
    }

    impl ProjectGroups {
        /// Get a project group by name
        pub fn get_by_name(
            &self,
            client: &dyn wadl::blocking::Client,
            name: &str,
        ) -> std::result::Result<ProjectGroupFull, Error> {
            let url = self.url().join(name).unwrap();
            ProjectGroup(url).get(client)
        }
    }

    impl Distributions {
        /// Get a distribution by name
        pub fn get_by_name(
            &self,
            client: &dyn wadl::blocking::Client,
            name: &str,
        ) -> std::result::Result<DistributionFull, Error> {
            let url = self.url().join(name).unwrap();
            Distribution(url).get(client)
        }
    }

    /// Represents either a Person or a Team
    pub enum PersonOrTeam {
        /// A person
        Person(Person),
        /// A team
        Team(Team),
    }

    impl People {
        /// Get a person or team by name
        pub fn get_by_name(
            &self,
            client: &dyn wadl::blocking::Client,
            name: &str,
        ) -> std::result::Result<PersonOrTeam, Error> {
            let url = self.url().join(&format!("~{}", name)).unwrap();

            let wadl = wadl::blocking::get_wadl_resource_by_href(client, &url)?;

            let types = wadl
                .r#type
                .iter()
                .filter_map(|t| t.id())
                .collect::<Vec<_>>();

            if types.contains(&"person") {
                Ok(PersonOrTeam::Person(Person(url)))
            } else if types.contains(&"team") {
                Ok(PersonOrTeam::Team(Team(url)))
            } else {
                Err(Error::InvalidUrl)
            }
        }
    }
}
