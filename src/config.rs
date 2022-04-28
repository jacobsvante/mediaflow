use reqwest::Url;

static DEFAULT_API_BASE_URL: &str = "https://customerapi.mediaflowpro.com/1/";

#[derive(Clone, Debug)]
pub struct Config {
    pub base_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new<T: ToString>(client_id: T, client_secret: T, username: T, password: T) -> Self {
        Self {
            base_url: Url::parse(DEFAULT_API_BASE_URL).unwrap(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}
