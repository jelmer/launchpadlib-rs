//! Launchpad-specific URIs and convenience lookup functions.
//!
//! The code in this module lets users say "staging" when they mean
//! "https://api.staging.launchpad.net/".

use std::collections::HashMap;

/// The root URL for the production Launchpad API.
pub const LPNET_SERVICE_ROOT: &str = "https://api.launchpad.net/";

/// The root URL for the QA staging Launchpad API.
pub const QASTAGING_SERVICE_ROOT: &str = "https://api.qastaging.launchpad.net/";

/// The root URL for the staging Launchpad API.
pub const STAGING_SERVICE_ROOT: &str = "https://api.staging.launchpad.net/";

/// The root URL for the development Launchpad API.
pub const DEV_SERVICE_ROOT: &str = "https://api.launchpad.test/";

/// The root URL for the dogfood Launchpad API.
pub const DOGFOOD_SERVICE_ROOT: &str = "https://api.dogfood.paddev.net/";

/// The root URL for the test development Launchpad API.
pub const TEST_DEV_SERVICE_ROOT: &str = "http://api.launchpad.test:8085/";

/// The web root URL for the production Launchpad.
pub const LPNET_WEB_ROOT: &str = "https://launchpad.net/";

/// The web root URL for the QA staging Launchpad.
pub const QASTAGING_WEB_ROOT: &str = "https://qastaging.launchpad.net/";

/// The web root URL for the staging Launchpad.
pub const STAGING_WEB_ROOT: &str = "https://staging.launchpad.net/";

/// The web root URL for the development Launchpad.
pub const DEV_WEB_ROOT: &str = "https://launchpad.test/";

/// The web root URL for the dogfood Launchpad.
pub const DOGFOOD_WEB_ROOT: &str = "https://dogfood.paddev.net/";

/// The web root URL for the test development Launchpad.
pub const TEST_DEV_WEB_ROOT: &str = "http://launchpad.test:8085/";

/// The root URL for the edge Launchpad API.
///
/// If you use EDGE_SERVICE_ROOT, or its alias, or the equivalent
/// string, launchpadlib will issue a deprecation warning and use
/// PRODUCTION_SERVICE_ROOT instead. Similarly for EDGE_WEB_ROOT.
pub const EDGE_SERVICE_ROOT: &str = "https://api.edge.launchpad.net/";

/// The web root URL for the edge Launchpad.
pub const EDGE_WEB_ROOT: &str = "https://edge.launchpad.net/";

lazy_static::lazy_static! {
    /// A mapping of server aliases to their respective service roots.
    pub static ref SERVICE_ROOTS: HashMap<&'static str, &'static str> = HashMap::from([
        ("production", LPNET_SERVICE_ROOT),
        ("edge", LPNET_SERVICE_ROOT),
        ("qastaging", QASTAGING_SERVICE_ROOT),
        ("staging", STAGING_SERVICE_ROOT),
        ("dogfood", DOGFOOD_SERVICE_ROOT),
        ("dev", DEV_SERVICE_ROOT),
        ("test_dev", TEST_DEV_SERVICE_ROOT),
    ]);

    /// A mapping of server aliases to their respective web roots.
    pub static ref WEB_ROOTS: HashMap<&'static str, &'static str> = HashMap::from([
        ("production", LPNET_WEB_ROOT),
        ("edge", LPNET_WEB_ROOT),
        ("qastaging", QASTAGING_WEB_ROOT),
        ("staging", STAGING_WEB_ROOT),
        ("dogfood", DOGFOOD_WEB_ROOT),
        ("dev", DEV_WEB_ROOT),
        ("test_dev", TEST_DEV_WEB_ROOT),
    ]);
}

#[derive(Debug)]
/// Error type for dereferencing aliases
pub enum Error {
    /// An invalid alias was provided
    InvalidAlias(String),

    /// An invalid URL was provided
    InvalidUrl(String),
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::InvalidUrl(err.to_string())
    }
}

/// Dereference what might a URL or an alias for a URL.
fn dereference_alias(root: &str, aliases: &HashMap<&str, &str>) -> Result<String, Error> {
    if root == "edge" {
        // This will trigger a deprecation warning and use production instead.
        log::warn!("Launchpad edge server no longer exists. Using 'production' instead.");
    }
    if let Some(alias) = aliases.get(root) {
        return Ok(alias.to_string());
    }

    // It's not an alias. Is it a valid URL?
    if let Ok(url) = url::Url::parse(root) {
        return Ok(url.to_string());
    }

    // It's not an alias or a valid URL.
    Err(Error::InvalidAlias(root.to_string()))
}

/// Dereference an alias to a service root.
///
/// A recognized server alias such as "staging" gets turned into the
/// appropriate URI. A URI gets returned as is. Any other string raises a
/// ValueError.
pub fn lookup_service_root(mut service_root: &str) -> Result<String, Error> {
    if service_root == EDGE_SERVICE_ROOT {
        // This will trigger a deprecation warning and use production instead.
        service_root = "edge";
    }
    dereference_alias(service_root, &SERVICE_ROOTS)
}

/// Dereference an alias to a website root.
///
/// A recognized server alias such as "staging" gets turned into the
/// appropriate URI. A URI gets returned as is. Any other string raises a
/// ValueError.
pub fn lookup_web_root(mut web_root: &str) -> Result<String, Error> {
    if web_root == EDGE_WEB_ROOT {
        // This will trigger a deprecation warning and use production instead.
        web_root = "edge";
    }
    dereference_alias(web_root, &WEB_ROOTS)
}

/// Turn a service root URL into a web root URL
///
/// This is done heuristically, not with a lookup.
pub fn web_root_for_service_root(service_root: &str) -> Result<String, Error> {
    let service_root = lookup_service_root(service_root)?;
    let mut web_root_uri: url::Url = service_root.parse()?;
    web_root_uri.set_path("");
    // Remove the first occurrence of "api."
    let new_host = web_root_uri
        .host_str()
        .as_ref()
        .map(|x| x.strip_prefix("api.").unwrap_or(x))
        .map(|x| x.to_string());
    web_root_uri.set_host(new_host.as_deref())?;
    let web_root = web_root_uri.to_string();
    // Ensure the web root ends with a slash
    let web_root = if web_root.ends_with('/') {
        web_root
    } else {
        format!("{}/", web_root)
    };
    Ok(web_root)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lookup_service_root() {
        use super::*;

        assert_eq!(
            lookup_service_root("staging").unwrap(),
            STAGING_SERVICE_ROOT
        );
        assert_eq!(
            lookup_service_root("https://api.launchpad.net/").unwrap(),
            LPNET_SERVICE_ROOT
        );
        assert!(lookup_service_root("invalid").is_err());
    }

    #[test]
    fn test_lookup_web_root() {
        use super::*;

        assert_eq!(lookup_web_root("staging").unwrap(), STAGING_WEB_ROOT);
        assert_eq!(
            lookup_web_root("https://launchpad.net/").unwrap(),
            LPNET_WEB_ROOT
        );
        assert!(lookup_web_root("invalid").is_err());
    }

    #[test]
    fn test_web_root_for_service_root() {
        use super::*;

        assert_eq!(
            web_root_for_service_root(STAGING_SERVICE_ROOT).unwrap(),
            STAGING_WEB_ROOT
        );
        assert_eq!(
            web_root_for_service_root(LPNET_SERVICE_ROOT).unwrap(),
            LPNET_WEB_ROOT
        );
        assert!(web_root_for_service_root("invalid").is_err());
    }

    #[test]
    fn test_dereference_alias() {
        use super::*;

        assert_eq!(
            dereference_alias("staging", &SERVICE_ROOTS).unwrap(),
            STAGING_SERVICE_ROOT
        );
        assert_eq!(
            dereference_alias("https://api.launchpad.net/", &SERVICE_ROOTS).unwrap(),
            LPNET_SERVICE_ROOT
        );
        assert!(dereference_alias("invalid", &SERVICE_ROOTS).is_err());
    }
}
