use actix_web::{get, web, HttpResponse, Responder};
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rand::Rng;
use reqwest::Client;
use crate::config::Config;

fn google_client(cfg: &Config) -> BasicClient {
    BasicClient::new(
        ClientId::new(cfg.google_client_id.clone()),
        Some(ClientSecret::new(cfg.google_client_secret.clone())),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(cfg.google_redirect_uri.clone()).unwrap())
}

#[get("/auth/google")]
pub async fn google_login(cfg: web::Data<Config>) -> impl Responder {
    let client = google_client(&cfg);

    let (auth_url, _csrf) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/auth/google/callback")]
pub async fn google_callback(
    cfg: web::Data<Config>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let code = query.get("code").unwrap().to_string();

    let client = google_client(&cfg);

    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .unwrap();

    let access_token = token.access_token().secret().to_string();

    // Fetch Google userinfo
    let client = Client::new();
    let userinfo: serde_json::Value = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    HttpResponse::Ok().json(userinfo)
}

