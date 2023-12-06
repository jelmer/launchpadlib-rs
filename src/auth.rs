//! Signing of requests for Launchpad.
//!
//! See the documentation at https://help.launchpad.net/API/SigningRequests for details

use chrono::{Utc};
use std::collections::HashMap;
use url::form_urlencoded;
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_encode};

use rand::Rng;

const REQUEST_TOKEN_URL: &str = "https://launchpad.net/+request-token";

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Parse(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::Request(error) => write!(f, "Request error: {}", error),
            Error::Parse(error) => write!(f, "Parse error: {}", error),
        }
    }
}

impl std::error::Error for Error {}

fn parse_token_response(response_text: &[u8]) -> (String, String) {
    let mut request_token = String::new();
    let mut request_token_secret = String::new();
    for (key, value) in form_urlencoded::parse(response_text) {
        if key == "oauth_token" {
            request_token = value.to_string();
        } else if key == "oauth_token_secret" {
            request_token_secret = value.to_string();
        } else {
            log::debug!("Unknown key in request token response: {}", key);
        }
    }

    (request_token, request_token_secret)
}

/// Get a request token and request token secret
pub fn get_request_token(consumer_key: &str) -> Result<(String, String), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("oauth_consumer_key", consumer_key);
    params.insert("oauth_signature_method", "PLAINTEXT");
    params.insert("oauth_signature", "&");

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(REQUEST_TOKEN_URL)
        .form(&params)
        .send()?;

    Ok(parse_token_response(&response.bytes()?))
}

/// Authorize a request token
pub fn authorize_token_url(
    oauth_token: &str,
    oauth_callback: Option<&url::Url>,
) -> Result<url::Url, url::ParseError> {
    let mut url: url::Url = "https://launchpad.net/+authorize-token".parse()?;
    url.query_pairs_mut().append_pair("oauth_token", oauth_token);
    if let Some(oauth_callback) = oauth_callback {
        url.query_pairs_mut()
            .append_pair("oauth_callback", oauth_callback.as_str());
    }

    Ok(url)
}

/// Exchange a request token for an access token
pub fn exchange_request_token(
    consumer_key: &str,
    consumer_secret: Option<&str>,
    request_token: &str,
    request_token_secret: Option<&str>,
) -> Result<(String, String), reqwest::Error> {
    // Prepare the request parameters
    let mut params = HashMap::new();
    params.insert("oauth_token", request_token);
    params.insert("oauth_consumer_key", consumer_key);
    params.insert("oauth_signature_method", "PLAINTEXT");
    let signature = calculate_plaintext_signature(
        consumer_secret,
        request_token_secret
    );
    params.insert("oauth_signature", signature.as_str());

    // Make a POST request to exchange the request token for an access token
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://launchpad.net/+access-token")
        .form(&params)
        .send()?;

    // Parse the response to get the access token and access token secret
    Ok(parse_token_response(&response.bytes()?))
}

fn calculate_plaintext_signature(consumer_secret: Option<&str>, token_secret: Option<&str>) -> String {
    let consumer_secret = consumer_secret.unwrap_or("");
    let token_secret = token_secret.unwrap_or("");

    let consumer_secret = percent_encode(consumer_secret.as_bytes(), RFC3986_UNRESERVED).to_string();
    let token_secret = percent_encode(token_secret.as_bytes(), RFC3986_UNRESERVED).to_string();

    format!("{}&{}", consumer_secret, token_secret)
}

struct OAuthAuthorizationHeader {
    realm: String,
    oauth_consumer_key: String,
    oauth_token: String,
    oauth_signature_method: String,
    oauth_signature: String,
    oauth_timestamp: String,
    oauth_nonce: String,
    oauth_version: String,
}

/// See https://oauth.net/core/1.0/#encoding_parameters
///
/// Launchpad seems to also not encode "+", ":" and "/" on their example page
const RFC3986_UNRESERVED: &AsciiSet = &NON_ALPHANUMERIC.remove(b'-').remove(b'.').remove(b'_').remove(b'~').remove(b'+').remove(b'/').remove(b'+').remove(b':');



impl std::str::FromStr for OAuthAuthorizationHeader {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("OAuth ").ok_or_else(|| {
            Error::Parse("Authorization header does not start with 'OAuth '".to_string())
        })?;

        let mut realm = String::new();
        let mut oauth_consumer_key = String::new();
        let mut oauth_token = String::new();
        let mut oauth_signature_method = String::new();
        let mut oauth_signature = String::new();
        let mut oauth_timestamp = String::new();
        let mut oauth_nonce = String::new();
        let mut oauth_version = String::new();

        for part in s.split(", ") {
            let mut parts = part.split('=');
            let key = parts.next().ok_or_else(|| Error::Parse("Missing key".to_string()))?;
            let value = parts.next().ok_or_else(|| Error::Parse("Missing value".to_string()))?;
            let value = value.trim_matches('"');

            let value = percent_encoding::percent_decode_str(value)
                .decode_utf8()
                .map_err(|e| Error::Parse(format!("Invalid UTF-8 in OAuth header: {}", e)))?
                .to_string();

            match key {
                "realm" => realm = value.to_string(),
                "oauth_consumer_key" => oauth_consumer_key = value.to_string(),
                "oauth_token" => oauth_token = value.to_string(),
                "oauth_signature_method" => oauth_signature_method = value.to_string(),
                "oauth_signature" => oauth_signature = value.to_string(),
                "oauth_timestamp" => oauth_timestamp = value.to_string(),
                "oauth_nonce" => oauth_nonce = value.to_string(),
                "oauth_version" => oauth_version = value.to_string(),
                _ => log::debug!("Unknown key in OAuth header: {}", key),
            }
        }

        Ok(OAuthAuthorizationHeader {
            realm,
            oauth_consumer_key,
            oauth_token,
            oauth_signature_method,
            oauth_signature,
            oauth_timestamp,
            oauth_nonce,
            oauth_version,
        })
    }
}

impl ToString for OAuthAuthorizationHeader {
    fn to_string(&self) -> String {
        let mut header = String::from("OAuth ");

        let push_str = |header: &mut String, key: &str, value: &str| {
            if header.as_str() != "OAuth " {
                header.push_str(", ");
            }
            header.push_str(key);
            header.push_str("=\"");
            header.push_str(percent_encode(value.as_bytes(), RFC3986_UNRESERVED).to_string().as_str());
            header.push('\"');
        };

        push_str(&mut header, "realm", &self.realm);
        push_str(&mut header, "oauth_consumer_key", &self.oauth_consumer_key);
        push_str(&mut header, "oauth_token", &self.oauth_token);
        push_str(&mut header, "oauth_signature_method", &self.oauth_signature_method);
        push_str(&mut header, "oauth_signature", &self.oauth_signature);
        push_str(&mut header, "oauth_timestamp", &self.oauth_timestamp);
        push_str(&mut header, "oauth_nonce", &self.oauth_nonce);
        push_str(&mut header, "oauth_version", &self.oauth_version);

        header
    }
}

/// Generate a string for use in the Authorize header:
pub fn generate_oauth1_authorization_header(
    url: &url::Url,
    consumer_key: &str,
    consumer_secret: Option<&str>,
    token: &str,
    token_secret: &str,
    timestamp: Option<chrono::NaiveDateTime>,
    nonce: Option<usize>,
) -> String {
    // Extract the first part of the URL, with the scheme and host
    let realm = format!("{}://{}/", url.scheme(), url.host_str().unwrap());

    let timestamp = timestamp
        .unwrap_or_else(|| Utc::now().naive_utc())
        .timestamp()
        .to_string();

    let nonce: String = nonce.map_or_else(
        || rand::thread_rng().gen_range(100000..999999).to_string(),
        |nonce| nonce.to_string());

    let signature = calculate_plaintext_signature(consumer_secret, Some(token_secret));

    OAuthAuthorizationHeader {
        realm,
        oauth_consumer_key: consumer_key.to_string(),
        oauth_token: token.to_string(),
        oauth_signature_method: "PLAINTEXT".to_string(),
        oauth_signature: signature,
        oauth_timestamp: timestamp,
        oauth_nonce: nonce,
        oauth_version: "1.0".to_string(),
    }.to_string()
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_generate_oauth1_authoriation_header() {
        let ret = super::generate_oauth1_authorization_header(
                &"https://api.launchpad.net/".
                    parse::<url::Url>().unwrap(),
                "just+testing",
                None,
                "PsK9cpbll1KwehhRDckr",
                "M2hsnmsfEIAjS3bTWg6t8X2GKhlm152PRDjLLmtQdr9C8KFZWPl9c8QbLfWddE0qpz5L56pMKKFKEfv1",
                Some(chrono::NaiveDateTime::from_timestamp(1217548916, 0)),
                Some(51769993));

        assert_eq!(
            "OAuth realm=\"https://api.launchpad.net/\", oauth_consumer_key=\"just+testing\", oauth_token=\"PsK9cpbll1KwehhRDckr\", oauth_signature_method=\"PLAINTEXT\", oauth_signature=\"%26M2hsnmsfEIAjS3bTWg6t8X2GKhlm152PRDjLLmtQdr9C8KFZWPl9c8QbLfWddE0qpz5L56pMKKFKEfv1\", oauth_timestamp=\"1217548916\", oauth_nonce=\"51769993\", oauth_version=\"1.0\"", ret.as_str());
    }

    #[test]
    fn test_authorize_token_url() {
        let ret = super::authorize_token_url(
            "9kDgVhXlcVn52HGgCWxq",
            None,
        );

        assert_eq!(
            "https://launchpad.net/+authorize-token?oauth_token=9kDgVhXlcVn52HGgCWxq",
            ret.unwrap().as_str());

        let ret = super::authorize_token_url(
            "9kDgVhXlcVn52HGgCWxq",
            Some(&"https://example.com/".parse().unwrap()),
        );

        assert_eq!(
            "https://launchpad.net/+authorize-token?oauth_token=9kDgVhXlcVn52HGgCWxq&oauth_callback=https%3A%2F%2Fexample.com%2F",
            ret.unwrap().as_str());
    }

    #[test]
    fn test_parse_token_response() {
        let ret = super::parse_token_response(
            b"oauth_token=9kDgVhXlcVn52HGgCWxq&oauth_token_secret=9kDgVhXlcVn52HGgCWxq",
        );

        assert_eq!(
            ("9kDgVhXlcVn52HGgCWxq".to_string(), "9kDgVhXlcVn52HGgCWxq".to_string()),
            ret);
    }
}
