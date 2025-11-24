use crate::config::Config;
use oauth2::{basic::BasicClient, ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl};

pub fn google_client(cfg: &Config) -> BasicClient {
    BasicClient::new(
        ClientId::new(cfg.google_client_id.clone()),
        Some(ClientSecret::new(cfg.google_client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(cfg.google_redirect_uri.clone()).unwrap())
}

