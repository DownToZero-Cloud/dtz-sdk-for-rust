use reqwest::redirect::Policy;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: Option<String>,
    pub client: reqwest::Client,
    pub api_key: Option<String>,
    pub oauth_access_token: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            base_path: None,
            client: reqwest::Client::builder()
                .redirect(Policy::none())
                .build()
                .unwrap_or_default(),
            api_key: None,
            oauth_access_token: None,
        }
    }
}
