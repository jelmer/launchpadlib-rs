//! Async version of the Launchpad API

#[allow(unused_imports)]
use crate::*;
pub mod client;
pub mod page;
pub use client::Client;

/// In development API
#[cfg(feature = "api-devel")]
pub mod devel {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    use super::*;
    use crate::page::AsTotalSize;
    use url::Url;

    include!(concat!(env!("OUT_DIR"), "/generated/async/devel.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/devel/").unwrap());
    }

    /// Get the default service root
    pub fn service_root(
        client: &dyn wadl::r#async::Client,
    ) -> std::result::Result<ServiceRootJson, wadl::Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust
    /// let client = launchpadlib::r#async::Client::anonymous("just+testing");
    /// let root = launchpadlib::r#async::devel::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::r#async::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, wadl::Error> {
        let url = Url::parse(&format!("https://{}/devel/", host)).unwrap();
        ServiceRoot(url).get(client)
    }
}

/// The current beta API
#[cfg(feature = "api-beta")]
pub mod beta {

    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    use super::*;
    use crate::page::AsTotalSize;
    use url::Url;

    include!(concat!(env!("OUT_DIR"), "/generated/async/beta.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/beta/").unwrap());
    }

    /// Get the default service root
    pub fn service_root(
        client: &dyn wadl::r#async::Client,
    ) -> std::result::Result<ServiceRootJson, wadl::Error> {
        ROOT.get(client)
    }

    /// Get the service root for a specific host
    ///
    /// # Example
    /// ```rust
    /// let client = launchpadlib::r#async::Client::anonymous("just+testing");
    /// let root = launchpadlib::r#async::beta::service_root_for_host(&client, "api.staging.launchpad.net").unwrap();
    /// ```
    pub fn service_root_for_host(
        client: &dyn wadl::r#async::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, wadl::Error> {
        let url = Url::parse(&format!("https://{}/beta/", host)).unwrap();
        ServiceRoot(url).get(client)
    }
}

/// The original version in the v1.0 API
#[cfg(feature = "api-v1_0")]
pub mod v1_0 {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    use super::*;
    use crate::page::AsTotalSize;
    use url::Url;

    include!(concat!(env!("OUT_DIR"), "/generated/async/1_0.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/1.0/").unwrap());
        static ref STATIC_RESOURCES: std::collections::HashMap<Url, Box<dyn Resource + Send + Sync>> = {
            let mut m = std::collections::HashMap::new();
            let root = ServiceRoot(Url::parse("https://api.launchpad.net/1.0/").unwrap());
            m.insert(root.url().clone(), Box::new(root) as Box<dyn Resource + Send + Sync>);
            m
        };
    }

    /// Get a service root by URL
    pub fn get_service_root_by_url(
        url: &'_ Url,
    ) -> std::result::Result<&'static ServiceRoot, wadl::Error> {
        if url == ROOT.url() {
            Ok(&ROOT)
        } else {
            Err(Error::InvalidUrl)
        }
    }

    /// Get the default service root
    pub async fn service_root(
        client: &dyn wadl::r#async::Client,
    ) -> std::result::Result<ServiceRootJson, wadl::Error> {
        ROOT.get(client).await
    }

    /// Get the service root for a specific host
    pub async fn service_root_for_host(
        client: &dyn wadl::r#async::Client,
        host: &str,
    ) -> std::result::Result<ServiceRootJson, wadl::Error> {
        let url = Url::parse(&format!("https://{}/1.0/", host)).unwrap();
        ServiceRoot(url).get(client).await
    }

    /// Get a resource by URL
    pub fn get_resource_by_url(
        url: &'_ Url,
    ) -> std::result::Result<&'static (dyn Resource + Send + Sync), wadl::Error> {
        if let Some(resource) = STATIC_RESOURCES.get(url) {
            Ok(resource.as_ref())
        } else {
            Err(Error::InvalidUrl)
        }
    }

    impl Bugs {
        /// Get a bug by its id
        pub async fn get_by_id(
            &self,
            client: &dyn wadl::r#async::Client,
            id: u32,
        ) -> std::result::Result<BugFull, wadl::Error> {
            let url = self.url().join(format!("bugs/{}", id).as_str()).unwrap();
            Bug(url).get(client).await
        }
    }

    impl Projects {
        /// Get a project by its name
        pub async fn get_by_name(
            &self,
            client: &dyn wadl::r#async::Client,
            name: &str,
        ) -> std::result::Result<ProjectFull, wadl::Error> {
            let url = self.url().join(name).unwrap();
            Project(url).get(client).await
        }
    }

    impl ProjectGroups {
        /// Get a project group by name
        pub async fn get_by_name(
            &self,
            client: &dyn wadl::r#async::Client,
            name: &str,
        ) -> std::result::Result<ProjectGroupFull, wadl::Error> {
            let url = self.url().join(name).unwrap();
            ProjectGroup(url).get(client).await
        }
    }

    impl Distributions {
        /// Get a distribution by name
        pub async fn get_by_name(
            &self,
            client: &dyn wadl::r#async::Client,
            name: &str,
        ) -> std::result::Result<DistributionFull, wadl::Error> {
            let url = self.url().join(name).unwrap();
            Distribution(url).get(client).await
        }
    }

    /// Enum for either a person or a team
    pub enum PersonOrTeam {
        /// A person
        Person(Person),

        /// A team
        Team(Team),
    }

    impl People {
        /// Get a person or team by name
        pub async fn get_by_name(
            &self,
            client: &dyn wadl::r#async::Client,
            name: &str,
        ) -> std::result::Result<PersonOrTeam, wadl::Error> {
            let url = self.url().join(&format!("~{}", name)).unwrap();

            let wadl = wadl::r#async::get_wadl_resource_by_href(client, &url).await?;

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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_person() {
            let json = include_str!("../../testdata/person.json");
            let person: PersonFull = serde_json::from_str(json).unwrap();
            assert_eq!(person.display_name, "Jelmer Vernooĳ");
        }

        #[test]
        fn test_parse_team() {
            let json = include_str!("../../testdata/team.json");
            let team: TeamFull = serde_json::from_str(json).unwrap();
            assert_eq!(team.display_name, "awsome-core");

            let json = include_str!("../../testdata/team2.json");
            let _team: TeamFull = serde_json::from_str(json).unwrap();
        }

        #[test]
        fn test_parse_bug() {
            let json = include_str!("../../testdata/bug.json");
            let bug: BugFull = serde_json::from_str(json).unwrap();
            assert_eq!(bug.title, "Microsoft has a majority market share");

            let json = include_str!("../../testdata/bug2.json");
            let bug: BugFull = serde_json::from_str(json).unwrap();
            assert_eq!(bug.name, None);
            assert_eq!(bug.id, 2039729);
        }

        #[test]
        fn test_parse_bug_tasks() {
            let json = include_str!("../../testdata/bug_tasks.json");
            let _bug_tasks: BugTaskPage = serde_json::from_str(json).unwrap();
        }
    }
}
