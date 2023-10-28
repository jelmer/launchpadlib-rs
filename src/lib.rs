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
//! let service_root = get_service_root_by_url(&url).unwrap().get().unwrap();
//! let people = service_root.people_collection().unwrap().unwrap();
//! let person = people.get_by_email("jelmer@jelmer.uk").unwrap();
//! println!("Person: {}", person.display_name);
//! ```

mod auth;
use url::Url;
use wadl::{Error, Resource};

#[cfg(feature = "api-devel")]
pub mod devel {
    #![allow(unused_mut)]
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated/devel.rs"));
}

#[cfg(feature = "api-beta")]
pub mod beta {
    #![allow(unused_mut)]
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/generated/beta.rs"));
}

#[cfg(feature = "api-v1_0")]
pub mod v1_0 {
    #![allow(unused_mut)]
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

pub struct Connection {
    client: reqwest::blocking::Client,
    consumer_key: Option<String>,
    consumer_secret: Option<String>,
    token: Option<String>,
    token_secret: Option<String>,
}

impl Connection {
    pub fn anonymous() -> Result<Self, Error> {
        Self::new(None, None, None, None, None)
    }

    pub fn new(
        consumer_key: Option<&str>,
        consumer_secret: Option<&str>,
        token: Option<&str>,
        token_secret: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<Self, Error> {
        let user_agent = user_agent.unwrap_or(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));
        let client = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .build()?;

        Ok(Self {
            client,
            token: token.map(|x| x.to_string()),
            token_secret: token_secret.map(|x| x.to_string()),
            consumer_key: consumer_key.map(|x| x.to_string()),
            consumer_secret: consumer_secret.map(|x| x.to_string()),
        })
    }

    fn authorization_header(&self, url: &Url, token: &str, token_secret: &str) -> String {
        crate::auth::generate_oauth1_authorization_header(
            url,
            self.consumer_key.as_ref().unwrap().as_str(),
            self.consumer_secret.as_ref().unwrap().as_str(),
            token,
            token_secret,
            None,
        )
    }

    pub fn request(
        &self,
        mut req: reqwest::blocking::Request,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        if let Some(token) = &self.token {
            let value = self.authorization_header(
                &req.url(),
                token.as_str(),
                self.token_secret.as_ref().unwrap().as_str(),
            );
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(value.as_str()).unwrap(),
            );
        }
        self.client.execute(req)
    }
}
