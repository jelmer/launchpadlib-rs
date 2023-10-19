//! # Launchpad API
//!
//! This crate provides a Rust interface to the Launchpad API.
//! It is generated from the Launchpad API WADL document.
//!
//! ## Usage
//! ```rust
//! use launchpad_api::v1_0::get_service_root_by_url;
//! use url::Url;
//!
//! let url = Url::parse("https://api.launchpad.net/1.0/").unwrap();
//! let service_root = get_service_root_by_url(&url).unwrap();
//! println!("Service root: {:?}", service_root);
//!
//! let url = Url::parse("https://api.launchpad.net/1.0/ubuntu/+archive/primary").unwrap();
#![allow(unused_mut)]
#[cfg(feature = "api-devel")]
pub mod devel {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated/devel.rs"));
}

#[cfg(feature = "api-beta")]
pub mod beta {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated/beta.rs"));
}

#[cfg(feature = "api-v1_0")]
pub mod v1_0 {
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

use url::Url;

/// The root of the web service.
pub trait Resource {
    fn url(&self) -> Url;
}

#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    Reqwest(reqwest::Error),
    Url(url::ParseError),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            Error::Url(err) => write!(f, "URL error: {}", err),
            Error::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}
