//! # Launchpad API
//!
//! This crate provides a Rust interface to the Launchpad API.
//! It is generated from the Launchpad API WADL document.
//!
//! ## Usage
//! ```rust
//! use launchpadlib::v1_0::get_service_root_by_url;
//! use url::Url;
//!
//! let url: Url = "https://api.launchpad.net/1.0/".parse().unwrap();
//! let client = launchpadlib::Client::anonymous("just+testing").unwrap();
//! let service_root = get_service_root_by_url(&url).unwrap().get(&client).unwrap();
//! let people = service_root.people().unwrap();
//! let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
//! println!("Person: {}", person.display_name);
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
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated/devel.rs"));
}

#[cfg(feature = "api-beta")]
pub mod beta {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated/beta.rs"));
}

#[cfg(feature = "api-v1_0")]
pub mod v1_0 {
    #![allow(unused_mut)]
    #![allow(clippy::too_many_arguments)]
    #![allow(clippy::wrong_self_convention)]
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/generated/1_0.rs"));

    #[derive(Clone)]
    struct ServiceRootResource1_0;
    impl Resource for ServiceRootResource1_0 {
        fn url(&self) -> Url {
            Url::parse("https://api.launchpad.net/1.0/").unwrap()
        }
    }
    impl ServiceRoot for ServiceRootResource1_0 {}

    lazy_static::lazy_static! {
        static ref STATIC_RESOURCES: std::collections::HashMap<Url, Box<dyn Resource + Send + Sync>> = {
            let mut m = std::collections::HashMap::new();
            m.insert(ServiceRootResource1_0.url(), Box::new(ServiceRootResource1_0) as Box<dyn Resource + Send + Sync>);
            m
        };
    }

    pub fn get_service_root_by_url(url: &'_ Url) -> Result<&'static (dyn ServiceRoot), Error> {
        if url == &ServiceRootResource1_0.url() {
            Ok(&ServiceRootResource1_0)
        } else {
            Err(Error::InvalidUrl)
        }
    }

    pub fn get_resource_by_url(
        url: &'_ Url,
    ) -> Result<&'static (dyn Resource + Send + Sync), Error> {
        if let Some(resource) = STATIC_RESOURCES.get(url) {
            Ok(resource.as_ref())
        } else {
            Err(Error::InvalidUrl)
        }
    }
}
