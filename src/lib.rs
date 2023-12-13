//! # Launchpad API
//!
//! This crate provides a Rust interface to the Launchpad API.
//! It is generated from the Launchpad API WADL document.
//!
//! ## Usage
//! ```rust
//! use url::Url;
//!
//! #[cfg(feature = "api-v1_0")]
//! {
//! let client = launchpadlib::Client::anonymous("just+testing").unwrap();
//! let service_root = launchpadlib::v1_0::service_root(&client).unwrap();
//! let people = service_root.people().unwrap();
//! let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
//! let ssh_keys = person.sshkeys(&client).unwrap().map(|k| k.unwrap().keytext).collect::<Vec<_>>();
//! println!("SSH Keys: {:?}", ssh_keys);
//! }
//! ```

pub mod auth;
pub mod client;
pub mod page;
pub use client::Client;

use url::Url;
use wadl::{Error, Resource};

#[cfg(feature = "api-devel")]
pub mod devel {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    #![allow(dead_code)]
    use super::*;
    use crate::page::AsTotalSize;
    include!(concat!(env!("OUT_DIR"), "/generated/devel.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/devel/").unwrap());
    }

    pub fn service_root(client: &dyn wadl::Client) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    pub fn service_root_for_host(client: &dyn wadl::Client, host: &str) -> std::result::Result<ServiceRootJson, Error> {
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
    use crate::page::AsTotalSize;
    include!(concat!(env!("OUT_DIR"), "/generated/beta.rs"));

    lazy_static::lazy_static! {
        static ref ROOT: ServiceRoot = ServiceRoot(Url::parse("https://api.launchpad.net/beta/").unwrap());
    }

    pub fn service_root(client: &dyn wadl::Client) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    pub fn service_root_for_host(client: &dyn wadl::Client, host: &str) -> std::result::Result<ServiceRootJson, Error> {
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
    use crate::page::AsTotalSize;

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

    pub fn get_service_root_by_url(url: &'_ Url) -> std::result::Result<&'static ServiceRoot, Error> {
        if url == ROOT.url() {
            Ok(&ROOT)
        } else {
            Err(Error::InvalidUrl)
        }
    }

    pub fn service_root(client: &dyn wadl::Client) -> std::result::Result<ServiceRootJson, Error> {
        ROOT.get(client)
    }

    pub fn service_root_for_host(client: &dyn wadl::Client, host: &str) -> std::result::Result<ServiceRootJson, Error> {
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

    #[test]
    fn test_parse_person() {
        let json = include_str!("../testdata/person.json");
        let person: PersonFull = serde_json::from_str(json).unwrap();
        assert_eq!(person.displayname, "Jelmer Vernooĳ");
    }

    impl Bugs {
        pub fn get_by_id(&self, client: &dyn wadl::Client, id: u32) -> std::result::Result<BugFull, Error> {
            let url = self.url().join(id.to_string().as_str()).unwrap();
            Bug(url).get(client)
        }
    }

    impl Projects {
        pub fn get_by_name(&self, client: &dyn wadl::Client, name: &str) -> std::result::Result<ProjectFull, Error> {
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
}
