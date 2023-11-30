extern crate oauth2;

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};

use std::env;
 

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Set up the config
    let client_id = ClientId::new(
        env::var("GOOGLE_OAUTH_CLIENT_ID").expect("Missing the CLIENT_ID environment variable."),
    );
    let client_secret = ClientSecret::new(
        env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("Missing the CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();

    let redirect_url = "http://localhost:8000/api/oauth2/google/callback";

    // Set up the OAuth2.0 client
    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.into()).expect("Invalid redirect URL"));

    // Generate the PKCE challenge and verifier.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
        .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Open this URL in your browser:\n{}\n", auth_url);

    // need this to swap for auth token later . prob need to store in db or something.
    println!(
        "code verifier secret is {}  . You will need this too for the next step.",
        pkce_verifier.secret()
    );

    // let code = AuthorizationCode::new("some authorization code".to_string()); // This should be provided by your OAuth2 provider.
}

//https://github.com/bk-rs/oauth2-rs/blob/main/oauth2-signin/src/web_app/signin_flow.rs
