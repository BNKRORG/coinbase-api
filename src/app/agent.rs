use std::time::Duration;

use reqwest::header::{CONTENT_TYPE, HeaderValue, USER_AGENT};
use reqwest::{Client, Method, Response};
use url::Url;

use super::auth::CoinbaseAuth;
use super::auth::jwt::Jwt;
use super::constant::{API_ROOT_URL, API_SANDBOX_URL, CB_VERSION, USER_AGENT_NAME};
use super::error::Error;
use super::response::CoinbaseErrorMessage;

#[derive(Debug, Clone)]
struct HttpClientAgent {
    /// Root URL for the API.
    root_url: Url,
    /// HTTP client.
    client: Client,
}

impl HttpClientAgent {
    fn new(use_sandbox: bool) -> Result<Self, Error> {
        let root_url: &str = if use_sandbox {
            API_SANDBOX_URL
        } else {
            API_ROOT_URL
        };

        let client = Client::builder()
            .timeout(Duration::from_secs(15)) // TODO: make configurable
            .build()?;

        Ok(Self {
            root_url: Url::parse(root_url)?,
            client,
        })
    }

    /// Constructs a URL for the request being made.
    fn build_url(&self, resource: &str, query: Option<&str>) -> Result<Url, Error> {
        let mut url = self.root_url.join(resource)?;
        url.set_query(query);
        Ok(url)
    }

    /// Handles the response from the API.
    async fn handle_response(&self, response: Response) -> Result<Response, Error> {
        if response.status().is_success() {
            Ok(response)
        } else {
            let res: CoinbaseErrorMessage = response.json().await?;
            Err(Error::Coinbase(res))
        }
    }

    pub(crate) async fn execute_request(
        &self,
        method: Method,
        url: Url,
        body: Option<String>,
        token: Option<String>,
    ) -> Result<Response, Error> {
        // {
        //     let mut locked_bucket = self.bucket.lock().await;
        //     locked_bucket.wait_on().await;
        // }

        let mut request = self
            .client
            .request(method, url)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, USER_AGENT_NAME)
            .header("CB-VERSION", HeaderValue::from_static(CB_VERSION));

        if let Some(token) = token {
            request = request.bearer_auth(token);
        }

        if let Some(body) = body {
            request = request.body(body);
        }

        let response = request.send().await?;

        self.handle_response(response).await
    }
}

#[derive(Debug, Clone)]
pub struct SecureHttpClientAgent {
    /// JWT generator, disabled in sandbox mode.
    jwt: Option<Jwt>,
    /// Base client that is responsible for making the requests.
    base: HttpClientAgent,
}

impl SecureHttpClientAgent {
    pub(super) fn new(auth: CoinbaseAuth, use_sandbox: bool) -> Result<Self, Error> {
        let jwt: Option<Jwt> = match auth {
            CoinbaseAuth::None => None,
            CoinbaseAuth::ApiKeys {
                api_key,
                secret_key,
            } => {
                // Do not generate JWT in sandbox mode.
                if use_sandbox {
                    None
                } else {
                    Some(Jwt::new(api_key, secret_key)?)
                }
            }
        };

        Ok(Self {
            jwt,
            base: HttpClientAgent::new(use_sandbox)?,
        })
    }

    /// Builds a token for the request.
    ///
    /// If JWT is not enabled, returns `None`.
    fn build_token(&self, method: &Method, resource: &str) -> Result<Option<String>, Error> {
        match &self.jwt {
            Some(jwt) => {
                let url: Url = self.base.root_url.join(resource)?;
                let uri: String = Jwt::build_uri(method, &url);
                Ok(Some(jwt.encode(Some(uri))?))
            }
            None => Ok(None),
        }
    }

    async fn get(&mut self, resource: &str, query: Option<&str>) -> Result<Response, Error> {
        const METHOD: Method = Method::GET;

        // Build URL
        let url: Url = self.base.build_url(resource, query)?;

        // Build token
        let token: Option<String> = self.build_token(&METHOD, resource)?;

        // Execute request
        self.base.execute_request(METHOD, url, None, token).await
    }
}
