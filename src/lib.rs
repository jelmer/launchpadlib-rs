#![deny(missing_docs)]
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
    fn as_total_size(self) -> Option<usize>;
}

impl AsTotalSize for Option<usize> {
    fn as_total_size(self) -> Option<usize> {
        self
    }
}

impl AsTotalSize for usize {
    fn as_total_size(self) -> Option<usize> {
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
}

#[cfg(feature = "api-devel")]
pub mod devel {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    use super::*;
    use crate::AsTotalSize;
    include!(concat!(env!("OUT_DIR"), "/generated/devel.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/devel/").unwrap());
    }

    /// Get the default service root
    pub fn service_root(client: &dyn wadl::Client) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust
    /// let client = launchpadlib::Client::anonymous("just+testing");
    /// let root = launchpadlib::devel::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, Error> {
        let url = Url::parse(&format!("https://{}/devel/", host)).unwrap();
        ServiceRoot(url).get(client)
    }
}

#[cfg(feature = "api-beta")]
pub mod beta {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    use super::*;
    use crate::AsTotalSize;
    include!(concat!(env!("OUT_DIR"), "/generated/beta.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/beta/").unwrap());
    }

    /// Get the default service root
    pub fn service_root(client: &dyn wadl::Client) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust
    /// let client = launchpadlib::Client::anonymous("just+testing");
    /// let root = launchpadlib::beta::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, Error> {
        let url = Url::parse(&format!("https://{}/beta/", host)).unwrap();
        ServiceRoot(url).get(client)
    }
}

#[cfg(feature = "api-v1_0")]
pub mod v1_0 {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    use super::*;
    use crate::AsTotalSize;

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
    pub fn service_root(client: &dyn wadl::Client) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust
    /// let client = launchpadlib::Client::anonymous("just+testing");
    /// let root = launchpadlib::v1_0::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, Error> {
        let url = Url::parse(&format!("https://{}/1.0/", host)).unwrap();
        ServiceRoot(url).get(client)
    }

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
            assert_eq!(person.display_name, "Jelmer Vernooĳ");
        }

        #[test]
        fn test_parse_team() {
            let json = include_str!("../testdata/team.json");
            let team: TeamFull = serde_json::from_str(json).unwrap();
            assert_eq!(team.display_name, "awsome-core");

            let json = include_str!("../testdata/team2.json");
            let team: TeamFull = serde_json::from_str(json).unwrap();
        }

        #[test]
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
        fn test_parse_bug_tasks() {
            let json = include_str!("../testdata/bug_tasks.json");
            let bug_tasks: BugTaskPage = serde_json::from_str(json).unwrap();
        }
    }

    impl Bugs {
        /// Get a bug by its id
        ///
        /// # Example
        /// ```rust
        /// let client = launchpadlib::Client::anonymous("just+testing");
        /// let root = launchpadlib::v1_0::service_root(&client).unwrap();
        /// let bug = root.bugs().unwrap().get_by_id(&client, 1).unwrap();
        /// ```
        pub fn get_by_id(
            &self,
            client: &dyn wadl::Client,
            id: u32,
        ) -> std::result::Result<BugFull, Error> {
            let url = self.url().join(format!("bugs/{}", id).as_str()).unwrap();
            Bug(url).get(client)
        }
    }

    impl Projects {
        pub fn get_by_name(
            &self,
            client: &dyn wadl::Client,
            name: &str,
        ) -> std::result::Result<ProjectFull, Error> {
            let url = self.url().join(name).unwrap();
            Project(url).get(client)
        }
    }

    impl ProjectGroups {
        pub fn get_by_name(
            &self,
            client: &dyn wadl::Client,
            name: &str,
        ) -> std::result::Result<ProjectGroupFull, Error> {
            let url = self.url().join(name).unwrap();
            ProjectGroup(url).get(client)
        }
    }

    impl Distributions {
        pub fn get_by_name(
            &self,
            client: &dyn wadl::Client,
            name: &str,
        ) -> std::result::Result<DistributionFull, Error> {
            let url = self.url().join(name).unwrap();
            Distribution(url).get(client)
        }
    }

    pub enum PersonOrTeam {
        Person(Person),
        Team(Team),
    }

    impl People {
        /// Get a person or team by name
        pub fn get_by_name(
            &self,
            client: &dyn wadl::Client,
            name: &str,
        ) -> std::result::Result<PersonOrTeam, Error> {
            let url = self.url().join(&format!("~{}", name)).unwrap();

            let wadl = wadl::get_wadl_resource_by_href(client, &url)?;

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
