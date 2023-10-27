use hmac::{Hmac, Mac};

use sha1::Sha1;
use std::collections::HashMap;
use url::form_urlencoded;

use chrono::Utc;
use rand::Rng;

/// Function to get the request token and request token secret
pub fn get_request_token(consumer_key: &str) -> Result<(String, String), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("oauth_consumer_key", consumer_key);
    params.insert("oauth_signature_method", "PLAINTEXT");
    params.insert("oauth_signature", "&");

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://launchpad.net/+request-token")
        .form(&params)
        .send()?;

    let response_text = response.text()?;
    let mut request_token = String::new();
    let mut request_token_secret = String::new();
    for (key, value) in form_urlencoded::parse(response_text.as_bytes()) {
        if key == "oauth_token" {
            request_token = value.to_string();
        } else if key == "oauth_token_secret" {
            request_token_secret = value.to_string();
        }
    }

    Ok((request_token, request_token_secret))
}

pub fn authorize_token(
    oauth_token: &str,
    oauth_callback: Option<&url::Url>,
) -> Result<(), reqwest::Error> {
    let mut params = HashMap::new();
    params.insert("oauth_token", oauth_token);
    if let Some(oauth_callback) = oauth_callback {
        params.insert("oauth_callback", oauth_callback.as_str());
    }

    let client = reqwest::blocking::Client::new();
    client
        .post("https://launchpad.net/+authorize-token")
        .form(&params)
        .send()?
        .error_for_status()?;

    Ok(())
}

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
        consumer_secret.unwrap_or(""),
        request_token_secret.unwrap_or(""),
    );
    params.insert("oauth_signature", signature.as_str());

    // Make a POST request to exchange the request token for an access token
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://launchpad.net/+access-token")
        .form(&params)
        .send()?;

    // Parse the response to get the access token and access token secret
    let response_text = response.text()?;
    let mut access_token = String::new();
    let mut access_token_secret = String::new();
    for (key, value) in form_urlencoded::parse(response_text.as_bytes()) {
        if key == "oauth_token" {
            access_token = value.to_string();
        } else if key == "oauth_token_secret" {
            access_token_secret = value.to_string();
        }
    }

    Ok((access_token, access_token_secret))
}

fn calculate_plaintext_signature(consumer_secret: &str, token_secret: &str) -> String {
    let key = format!("{}&{}", consumer_secret, token_secret);

    let mac = Hmac::<Sha1>::new_from_slice(key.as_bytes()).expect("HMAC initialization failed");

    let result = mac.finalize().into_bytes();

    let signature = format!("{:x}", result);

    signature
}

pub fn generate_oauth1_authorization_header(
    url: &url::Url,
    consumer_key: &str,
    consumer_secret: &str,
    token: &str,
    token_secret: &str,
) -> String {
    // Extract the first part of the URL, with the scheme and host
    let realm = format!("{}://{}/", url.scheme(), url.host_str().unwrap());

    let timestamp = Utc::now().timestamp().to_string();

    let nonce: String = rand::thread_rng().gen_range(100000..999999).to_string();

    let signature = calculate_plaintext_signature(consumer_secret, token_secret);

    let mut oauth_params = HashMap::new();
    oauth_params.insert("realm", realm.as_str());
    oauth_params.insert("oauth_consumer_key", consumer_key);
    oauth_params.insert("oauth_token", token);
    oauth_params.insert("oauth_signature_method", "PLAINTEXT");
    oauth_params.insert("oauth_signature", signature.as_str());
    oauth_params.insert("oauth_timestamp", &timestamp);
    oauth_params.insert("oauth_nonce", &nonce);
    oauth_params.insert("oauth_version", "1.0");

    let oauth_string = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(oauth_params.iter())
        .finish();

    let authorization_header = format!("OAuth {}", oauth_string.replace('&', ", "));

    authorization_header
}
