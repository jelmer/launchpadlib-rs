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
