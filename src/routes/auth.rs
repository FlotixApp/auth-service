use axum::{extract::State, response::Redirect, Json};
use tower_sessions::Session;
use oauth2::{CsrfToken, Scope};
use serde::Serialize;

use crate::{
    config::Config,
    providers::{google::google_client, github::github_client, apple::apple_client_secret},
};

#[derive(Clone)]
pub struct AppState {
    pub cfg: Config,
}

//
// GOOGLE LOGIN
//

pub async fn google_login(State(state): State<AppState>, session: Session) -> Redirect {
    let client = google_client(&state.cfg);
    let (auth_url, csrf) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .url();

    session.insert("google_csrf", csrf.secret()).await.unwrap();
    Redirect::to(auth_url.as_str())
}

pub async fn google_callback(
    State(state): State<AppState>,
    session: Session,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>
) -> Json<impl Serialize> {
    let saved_csrf: String = session.get("google_csrf").await.unwrap().unwrap();
    let code = params["code"].clone();
    let csrf = params["state"].clone();

    if csrf != saved_csrf {
        panic!("Invalid CSRF");
    }

    let client = google_client(&state.cfg);

    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .unwrap();

    let id_token = token
        .extra_fields()
        .id_token()
        .expect("ID token missing");

    let claims = id_token.decode(None, &state.cfg.google_client_id).unwrap();

    // store session
    session.insert("user", claims.claims().email.clone()).await.unwrap();

    Json(serde_json::json!({ "status": "ok", "email": claims.claims().email }))
}

//
// GITHUB LOGIN
//

pub async fn github_login(State(state): State<AppState>, session: Session) -> Redirect {
    let client = github_client(&state.cfg);
    let (auth_url, csrf) = client.authorize_url(CsrfToken::new_random).url();

    session.insert("github_csrf", csrf.secret()).await.unwrap();
    Redirect::to(auth_url.as_str())
}

pub async fn github_callback(
    State(state): State<AppState>,
    session: Session,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>
) -> Json<impl Serialize> {
    let saved_csrf: String = session.get("github_csrf").await.unwrap().unwrap();
    let code = params["code"].clone();
    let csrf = params["state"].clone();

    if csrf != saved_csrf {
        panic!("Invalid CSRF");
    }

    let client = github_client(&state.cfg);

    let token = client
        .exchange_code(oauth2::AuthorizationCode::new(code))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .unwrap();

    let access_token = token.access_token().secret();

    // Fetch user info from GitHub
    let user: serde_json::Value = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("User-Agent", "flotix-auth")
        .bearer_auth(access_token)
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();

    session.insert("user", user["login"].as_str()).await.unwrap();

    Json(serde_json::json!({
        "status": "ok",
        "user": user
    }))
}


//
// APPLE LOGIN (OIDC)
//

pub async fn apple_login(State(state): State<AppState>) -> Redirect {
    let secret = apple_client_secret(&state.cfg);

    let auth_url =
        format!(
            "https://appleid.apple.com/auth/authorize?client_id={}&redirect_uri={}&response_type=code id_token&scope=email name&response_mode=form_post",
            state.cfg.apple_client_id,
            urlencoding::encode(&state.cfg.apple_redirect_uri),
        );

    Redirect::to(&auth_url)
}


