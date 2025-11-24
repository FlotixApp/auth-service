use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,

    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_uri: String,

    pub apple_client_id: String,
    pub apple_team_id: String,
    pub apple_key_id: String,
    pub apple_private_key: String,
    pub apple_redirect_uri: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();
        Self {
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI").unwrap(),

            github_client_id: env::var("GITHUB_CLIENT_ID").unwrap(),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET").unwrap(),
            github_redirect_uri: env::var("GITHUB_REDIRECT_URI").unwrap(),

            apple_client_id: env::var("APPLE_CLIENT_ID").unwrap(),
            apple_team_id: env::var("APPLE_TEAM_ID").unwrap(),
            apple_key_id: env::var("APPLE_KEY_ID").unwrap(),
            apple_private_key: env::var("APPLE_PRIVATE_KEY").unwrap(),
            apple_redirect_uri: env::var("APPLE_REDIRECT_URI").unwrap(),
        }
    }
}

