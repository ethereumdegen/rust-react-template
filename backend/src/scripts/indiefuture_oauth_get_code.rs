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
    let oauth_client_id = ClientId::new(
        env::var("INDIEFUTURE_OAUTH_CLIENT_ID")
            .expect("Missing the CLIENT_ID environment variable."),
    );
    let oauth_client_secret = ClientSecret::new(
        env::var("INDIEFUTURE_OAUTH_CLIENT_SECRET")
            .expect("Missing the CLIENT_SECRET environment variable."),
    );

    let indiefuture_frontend_domain = "https://indiefuture.com";
    let indiefuture_backend_domain = "https://api.indiefuture.com";
    let oauth2_auth_url = format!("{}/oauth2/authorize", indiefuture_frontend_domain);

    let oauth2_token_url = format!("{}/api/v1/oauth2/token", indiefuture_backend_domain);

    let auth_url = AuthUrl::new(oauth2_auth_url.into()).unwrap();
    let token_url = TokenUrl::new(oauth2_token_url.into()).unwrap();

    let tempova_backend_domain = "https://api.tempova.com";

    let redirect_url = format!("{}/api/oauth2/indiefuture/callback", tempova_backend_domain);

    //this is a tempova backend redirect url !
    // let redirect_url = "http://localhost:9000/api/oauth2/indiefuture/callback";

    // Set up the OAuth2.0 client
    let client = BasicClient::new(
        oauth_client_id,
        Some(oauth_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.into()).expect("Invalid redirect URL"));

    // Generate the PKCE challenge and verifier.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        /*  .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))  */
        .add_scope(Scope::new("offline.access".to_string()))
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
