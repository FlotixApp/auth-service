use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();
        Self {
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI").unwrap(),
        }
    }
}

