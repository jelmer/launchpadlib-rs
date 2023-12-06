use url::Url;
use crate::Error;

pub struct Client {
    client: reqwest::blocking::Client,
    consumer_key: Option<String>,
    consumer_secret: Option<String>,
    token: Option<String>,
    token_secret: Option<String>,
}

impl Client {
    /// Create a new client with no credentials.
    pub fn anonymous(consumer_key: &str) -> Result<Self, Error> {
        Self::new(Some(consumer_key), None, None, None, None)
    }

    /// Create a new client with the given credentials.
    pub fn authenticated(
        consumer_key: &str,
        consumer_secret: Option<&str>,
        token: &str,
        token_secret: &str,
    ) -> Result<Self, Error> {
        Self::new(
            Some(consumer_key),
            consumer_secret,
            Some(token),
            Some(token_secret),
            None,
        )
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

    /// Generate an OAuth1 authorization header for the given URL.
    fn authorization_header(&self, url: &Url, token: &str, token_secret: &str) -> String {
        crate::auth::generate_oauth1_authorization_header(
            url,
            self.consumer_key.as_ref().unwrap().as_str(),
            self.consumer_secret.as_deref(),
            token,
            token_secret,
            None,
            None
        )
    }
}

impl wadl::Client for Client {

    /// Perform a request, adding the appropriate OAuth1 headers.
    fn execute(
        &self,
        mut req: reqwest::blocking::Request,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        if let Some(token) = &self.token {
            let value = self.authorization_header(
                req.url(),
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
