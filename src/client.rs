//! The `Client` struct is a wrapper around `reqwest::blocking::Client` that provides OAuth1
//! authentication for requests.

use url::Url;

/// A client that can make requests to a Launchpad API.
///
/// This client is a wrapper around `reqwest::blocking::Client` that provides OAuth1 authentication
/// for requests. It can be created with or without credentials, and can be used to make requests
/// to any Launchpad API.
pub struct Client {
    client: reqwest::blocking::Client,
    consumer_key: Option<String>,
    consumer_secret: Option<String>,
    token: Option<String>,
    token_secret: Option<String>,
}

impl Client {
    /// Create a new client with no credentials.
    pub fn anonymous(consumer_key: &str) -> Self {
        Self::new(Some(consumer_key), None, None, None, None)
    }

    /// Create a new client with the given credentials.
    pub fn from_tokens(
        consumer_key: &str,
        consumer_secret: Option<&str>,
        token: &str,
        token_secret: &str,
    ) -> Self {
        Self::new(
            Some(consumer_key),
            consumer_secret,
            Some(token),
            Some(token_secret),
            None,
        )
    }

    /// Create a new client with the given credentials.
    pub fn authenticated(
        instance: Option<&str>,
        consumer_key: &str,
    ) -> Result<Self, crate::auth::Error> {
        let instance = instance.unwrap_or(crate::DEFAULT_INSTANCE);
        let (token, token_secret) = auth::get_access_token(instance, consumer_key)?;
        Ok(Self::from_tokens(consumer_key, None, &token, &token_secret))
    }

    /// Create a new client with the given credentials.
    pub fn new(
        consumer_key: Option<&str>,
        consumer_secret: Option<&str>,
        token: Option<&str>,
        token_secret: Option<&str>,
        user_agent: Option<&str>,
    ) -> Self {
        let user_agent = user_agent.unwrap_or(crate::DEFAULT_USER_AGENT);
        let client = reqwest::blocking::Client::builder()
            .user_agent(user_agent)
            .build()
            .unwrap();

        Self {
            client,
            token: token.map(|x| x.to_string()),
            token_secret: token_secret.map(|x| x.to_string()),
            consumer_key: consumer_key.map(|x| x.to_string()),
            consumer_secret: consumer_secret.map(|x| x.to_string()),
        }
    }

    /// Generate an OAuth1 authorization header for the given URL.
    fn authorization_header(&self, url: &Url, token: &str, token_secret: &str) -> String {
        crate::auth::generate_oauth1_authorization_header(
            url,
            self.consumer_key.as_ref().unwrap().as_str(),
            self.consumer_secret.as_deref(),
            token,
            token_secret,
            None,
            None,
        )
    }
}

impl wadl::blocking::Client for Client {
    fn request(&self, method: reqwest::Method, url: url::Url) -> reqwest::blocking::RequestBuilder {
        let auth_header = self.token.as_ref().map(|token| {
            self.authorization_header(
                &url,
                token.as_str(),
                self.token_secret.as_ref().unwrap().as_str(),
            )
        });
        let mut builder = self.client.request(method, url);

        if let Some(value) = auth_header {
            builder = builder.header(reqwest::header::AUTHORIZATION, value);
        }

        builder
    }
}

/// OAuth1 authentication functions
pub mod auth {
    use std::collections::HashMap;

    /// Exchange a request token for an access token
    pub fn exchange_request_token(
        instance: &str,
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
        let signature =
            crate::auth::calculate_plaintext_signature(consumer_secret, request_token_secret);
        params.insert("oauth_signature", signature.as_str());

        let mut url = url::Url::parse(crate::auth::ACCESS_TOKEN_URL).unwrap();

        url.set_host(Some(instance)).unwrap();

        // Make a POST request to exchange the request token for an access token
        let client = reqwest::blocking::Client::new();
        let response = client.post(url).form(&params).send()?;

        // Parse the response to get the access token and access token secret
        Ok(crate::auth::parse_token_response(&response.bytes()?))
    }

    #[cfg(feature = "keyring")]
    /// Obtain an access token from either the keyring, or by prompting the user
    pub fn keyring_access_token(
        instance: &str,
        consumer_key: &str,
    ) -> Result<(String, String), Error> {
        let entry = keyring::Entry::new(instance, "oauth1")?;

        let access_token = match entry.get_password() {
            Ok(token) => {
                log::debug!("Found entry in keyring for {}", instance);
                let (token, secret) = parse_token_response(token.as_bytes());
                log::debug!("Parsed token: {} / {}", token, secret);
                (token, secret)
            }
            Err(keyring::Error::NoEntry) => {
                log::debug!("No entry found in keyring at {}", instance);

                // Step 1: Get a request token
                let req_token = get_request_token(instance, consumer_key)?;

                // Step 2: Get the user to authorize the request token
                let auth_url =
                    crate::auth::authorize_token_url(instance, req_token.0.as_str(), None)?;

                println!("Please authorize the request token at {}", auth_url);
                println!("Once done, press enter to continue...");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                // Step 3: Exchange the request token for an access token
                let access_token = exchange_request_token(
                    instance,
                    consumer_key,
                    None,
                    req_token.0.as_str(),
                    Some(req_token.1.as_str()),
                )?;

                entry.set_password(&format!(
                    "oauth_token={}&oauth_token_secret={}",
                    access_token.0, access_token.1
                ))?;

                access_token
            }
            Err(e) => return Err(e.into()),
        };

        Ok(access_token)
    }

    /// Get an access token from the Launchpad API, by prompting the user for input on the command line
    ///
    /// # Arguments
    /// * `instance` - The Launchpad instance to use, or `None` for the default
    /// * `consumer_key` - The consumer key to use
    pub fn cmdline_access_token(
        instance: &str,
        consumer_key: &str,
    ) -> Result<(String, String), reqwest::Error> {
        // Step 1: Get a request token
        let req_token = get_request_token(instance, consumer_key)?;

        // Step 2: Get the user to authorize the request token
        let auth_url =
            crate::auth::authorize_token_url(instance, req_token.0.as_str(), None).unwrap();

        println!("Please authorize the request token at {}", auth_url);
        println!("Once done, press enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Step 3: Exchange the request token for an access token
        exchange_request_token(
            instance,
            consumer_key,
            None,
            req_token.0.as_str(),
            Some(req_token.1.as_str()),
        )
    }

    /// Get a request token and request token secret
    pub fn get_request_token(
        instance: &str,
        consumer_key: &str,
    ) -> Result<(String, String), reqwest::Error> {
        let params = crate::auth::request_token_params(consumer_key);

        let mut url = url::Url::parse(crate::auth::REQUEST_TOKEN_URL).unwrap();

        url.set_host(Some(instance)).unwrap();

        let client = reqwest::blocking::Client::new();
        let response = client.post(url).form(&params).send()?;

        Ok(crate::auth::parse_token_response(&response.bytes()?))
    }

    #[cfg(feature = "keyring")]
    /// Get an access token, either from the keyring or by prompting the user
    pub fn get_access_token(instance: &str, consumer_key: &str) -> Result<(String, String), Error> {
        keyring_access_token(instance, consumer_key)
    }

    #[cfg(not(feature = "keyring"))]
    /// Get an access token, by prompting the user
    pub fn get_access_token(
        instance: &str,
        consumer_key: &str,
    ) -> Result<(String, String), reqwest::Error> {
        cmdline_access_token(instance, consumer_key)
    }
}
