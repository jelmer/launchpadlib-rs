//! The `Client` struct is a wrapper around `reqwest::Client` that provides OAuth1
//! authentication for requests.

use url::Url;

/// A client that can make requests to a Launchpad API.
///
/// This client is a wrapper around `reqwest::Client` that provides OAuth1 authentication
/// for requests. It can be created with or without credentials, and can be used to make requests
/// to any Launchpad API.
pub struct Client {
    client: reqwest::Client,
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
        let (token, token_secret) = crate::auth::get_access_token(instance, consumer_key)?;
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
        let client = reqwest::Client::builder()
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

#[async_trait::async_trait]
impl wadl::r#async::Client for Client {
    async fn request(&self, method: reqwest::Method, url: url::Url) -> reqwest::RequestBuilder {
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
