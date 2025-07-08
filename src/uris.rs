//! Launchpad-specific URIs and convenience lookup functions.
//!
//! The code in this module lets users say "staging" when they mean
//! "https://api.staging.launchpad.net/".

use std::collections::HashMap;
use std::fmt;

/// Enum representing different Launchpad environments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaunchpadEnvironment {
    /// Production environment
    Production,
    /// Edge environment (deprecated, redirects to production)
    Edge,
    /// QA staging environment
    QaStaging,
    /// Staging environment
    Staging,
    /// Dogfood environment
    Dogfood,
    /// Development environment
    Dev,
    /// Test development environment
    TestDev,
}

impl LaunchpadEnvironment {
    /// Get the service root URL for this environment
    pub fn service_root(&self) -> &'static str {
        match self {
            LaunchpadEnvironment::Production => LPNET_SERVICE_ROOT,
            LaunchpadEnvironment::Edge => LPNET_SERVICE_ROOT,
            LaunchpadEnvironment::QaStaging => QASTAGING_SERVICE_ROOT,
            LaunchpadEnvironment::Staging => STAGING_SERVICE_ROOT,
            LaunchpadEnvironment::Dogfood => DOGFOOD_SERVICE_ROOT,
            LaunchpadEnvironment::Dev => DEV_SERVICE_ROOT,
            LaunchpadEnvironment::TestDev => TEST_DEV_SERVICE_ROOT,
        }
    }

    /// Get the web root URL for this environment
    pub fn web_root(&self) -> &'static str {
        match self {
            LaunchpadEnvironment::Production => LPNET_WEB_ROOT,
            LaunchpadEnvironment::Edge => LPNET_WEB_ROOT,
            LaunchpadEnvironment::QaStaging => QASTAGING_WEB_ROOT,
            LaunchpadEnvironment::Staging => STAGING_WEB_ROOT,
            LaunchpadEnvironment::Dogfood => DOGFOOD_WEB_ROOT,
            LaunchpadEnvironment::Dev => DEV_WEB_ROOT,
            LaunchpadEnvironment::TestDev => TEST_DEV_WEB_ROOT,
        }
    }

    /// Get the alias string for this environment
    pub fn alias(&self) -> &'static str {
        match self {
            LaunchpadEnvironment::Production => "production",
            LaunchpadEnvironment::Edge => "edge",
            LaunchpadEnvironment::QaStaging => "qastaging",
            LaunchpadEnvironment::Staging => "staging",
            LaunchpadEnvironment::Dogfood => "dogfood",
            LaunchpadEnvironment::Dev => "dev",
            LaunchpadEnvironment::TestDev => "test_dev",
        }
    }

    /// Try to parse an environment from an alias string
    pub fn from_alias(alias: &str) -> Option<Self> {
        match alias {
            "production" => Some(LaunchpadEnvironment::Production),
            "edge" => Some(LaunchpadEnvironment::Edge),
            "qastaging" => Some(LaunchpadEnvironment::QaStaging),
            "staging" => Some(LaunchpadEnvironment::Staging),
            "dogfood" => Some(LaunchpadEnvironment::Dogfood),
            "dev" => Some(LaunchpadEnvironment::Dev),
            "test_dev" => Some(LaunchpadEnvironment::TestDev),
            _ => None,
        }
    }

    /// Try to determine the environment from a URL
    pub fn from_url(url: &url::Url) -> Option<Self> {
        let host = url.host_str()?;
        let port = url.port();

        // Check for TestDev first (it has a specific port)
        if port == Some(8085) && (host == "launchpad.test" || host == "api.launchpad.test") {
            return Some(LaunchpadEnvironment::TestDev);
        }

        match host {
            "launchpad.net" | "api.launchpad.net" => Some(LaunchpadEnvironment::Production),
            "edge.launchpad.net" | "api.edge.launchpad.net" => Some(LaunchpadEnvironment::Edge),
            "staging.launchpad.net" | "api.staging.launchpad.net" => {
                Some(LaunchpadEnvironment::Staging)
            }
            "qastaging.launchpad.net" | "api.qastaging.launchpad.net" => {
                Some(LaunchpadEnvironment::QaStaging)
            }
            "dogfood.paddev.net" | "api.dogfood.paddev.net" => Some(LaunchpadEnvironment::Dogfood),
            "launchpad.test" | "api.launchpad.test" => Some(LaunchpadEnvironment::Dev),
            _ => None,
        }
    }

    /// Get all known Launchpad hosts for this environment
    pub fn hosts(&self) -> &'static [&'static str] {
        match self {
            LaunchpadEnvironment::Production => &["launchpad.net", "api.launchpad.net"],
            LaunchpadEnvironment::Edge => &["edge.launchpad.net", "api.edge.launchpad.net"],
            LaunchpadEnvironment::QaStaging => {
                &["qastaging.launchpad.net", "api.qastaging.launchpad.net"]
            }
            LaunchpadEnvironment::Staging => {
                &["staging.launchpad.net", "api.staging.launchpad.net"]
            }
            LaunchpadEnvironment::Dogfood => &["dogfood.paddev.net", "api.dogfood.paddev.net"],
            LaunchpadEnvironment::Dev => &["launchpad.test", "api.launchpad.test"],
            LaunchpadEnvironment::TestDev => &["launchpad.test", "api.launchpad.test"],
        }
    }

    /// Get all environments as a slice
    pub fn all() -> &'static [LaunchpadEnvironment] {
        &[
            LaunchpadEnvironment::Production,
            LaunchpadEnvironment::Edge,
            LaunchpadEnvironment::QaStaging,
            LaunchpadEnvironment::Staging,
            LaunchpadEnvironment::Dogfood,
            LaunchpadEnvironment::Dev,
            LaunchpadEnvironment::TestDev,
        ]
    }
}

impl fmt::Display for LaunchpadEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.alias())
    }
}

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

/// Check if a URL is a Launchpad URL by checking if it matches any known environment
pub fn is_launchpad_url(url: &url::Url) -> bool {
    LaunchpadEnvironment::from_url(url).is_some()
}

/// Get all known Launchpad hosts from all environments
pub fn get_known_launchpad_hosts() -> Vec<&'static str> {
    let mut all_hosts = Vec::new();

    for env in LaunchpadEnvironment::all() {
        all_hosts.extend(env.hosts());
    }

    all_hosts.sort_unstable();
    all_hosts.dedup();
    all_hosts
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

    #[test]
    fn test_environment_from_url() {
        use super::*;

        // Test production URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(&url::Url::parse("https://launchpad.net/").unwrap()),
            Some(LaunchpadEnvironment::Production)
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(&url::Url::parse("https://api.launchpad.net/").unwrap()),
            Some(LaunchpadEnvironment::Production)
        );

        // Test staging URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://staging.launchpad.net/").unwrap()
            ),
            Some(LaunchpadEnvironment::Staging)
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://api.staging.launchpad.net/").unwrap()
            ),
            Some(LaunchpadEnvironment::Staging)
        );

        // Test QA staging URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://qastaging.launchpad.net/").unwrap()
            ),
            Some(LaunchpadEnvironment::QaStaging)
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://api.qastaging.launchpad.net/").unwrap()
            ),
            Some(LaunchpadEnvironment::QaStaging)
        );

        // Test dogfood URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://dogfood.paddev.net/").unwrap()
            ),
            Some(LaunchpadEnvironment::Dogfood)
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://api.dogfood.paddev.net/").unwrap()
            ),
            Some(LaunchpadEnvironment::Dogfood)
        );

        // Test dev URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(&url::Url::parse("https://launchpad.test/").unwrap()),
            Some(LaunchpadEnvironment::Dev)
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("https://api.launchpad.test/").unwrap()
            ),
            Some(LaunchpadEnvironment::Dev)
        );

        // Test test dev URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("http://launchpad.test:8085/").unwrap()
            ),
            Some(LaunchpadEnvironment::TestDev)
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(
                &url::Url::parse("http://api.launchpad.test:8085/").unwrap()
            ),
            Some(LaunchpadEnvironment::TestDev)
        );

        // Test non-Launchpad URLs
        assert_eq!(
            LaunchpadEnvironment::from_url(&url::Url::parse("https://github.com/").unwrap()),
            None
        );
        assert_eq!(
            LaunchpadEnvironment::from_url(&url::Url::parse("https://example.com/").unwrap()),
            None
        );
    }

    #[test]
    fn test_environment_hosts() {
        use super::*;

        let prod_hosts = LaunchpadEnvironment::Production.hosts();
        assert!(prod_hosts.contains(&"launchpad.net"));
        assert!(prod_hosts.contains(&"api.launchpad.net"));

        let staging_hosts = LaunchpadEnvironment::Staging.hosts();
        assert!(staging_hosts.contains(&"staging.launchpad.net"));
        assert!(staging_hosts.contains(&"api.staging.launchpad.net"));
    }

    #[test]
    fn test_is_launchpad_url() {
        use super::*;

        // Test valid Launchpad URLs
        assert!(is_launchpad_url(
            &url::Url::parse("https://launchpad.net/ubuntu").unwrap()
        ));
        assert!(is_launchpad_url(
            &url::Url::parse("https://api.launchpad.net/1.0/").unwrap()
        ));
        assert!(is_launchpad_url(
            &url::Url::parse("https://staging.launchpad.net/").unwrap()
        ));
        assert!(is_launchpad_url(
            &url::Url::parse("https://api.staging.launchpad.net/").unwrap()
        ));
        assert!(is_launchpad_url(
            &url::Url::parse("https://dogfood.paddev.net/").unwrap()
        ));
        assert!(is_launchpad_url(
            &url::Url::parse("http://launchpad.test:8085/").unwrap()
        ));

        // Test non-Launchpad URLs
        assert!(!is_launchpad_url(
            &url::Url::parse("https://github.com/").unwrap()
        ));
        assert!(!is_launchpad_url(
            &url::Url::parse("https://example.com/").unwrap()
        ));
        assert!(!is_launchpad_url(
            &url::Url::parse("https://not-launchpad.net/").unwrap()
        ));
    }

    #[test]
    fn test_get_known_launchpad_hosts() {
        use super::*;

        let hosts = get_known_launchpad_hosts();

        // Check that we have some hosts
        assert!(!hosts.is_empty());

        // Check for some expected hosts
        assert!(hosts.contains(&"launchpad.net"));
        assert!(hosts.contains(&"api.launchpad.net"));
        assert!(hosts.contains(&"staging.launchpad.net"));
        assert!(hosts.contains(&"api.staging.launchpad.net"));
        assert!(hosts.contains(&"dogfood.paddev.net"));
        assert!(hosts.contains(&"api.dogfood.paddev.net"));

        // Check that hosts are sorted
        let mut sorted = hosts.clone();
        sorted.sort_unstable();
        assert_eq!(hosts, sorted);

        // Check no duplicates
        let mut deduped = hosts.clone();
        deduped.dedup();
        assert_eq!(hosts.len(), deduped.len());
    }
}
