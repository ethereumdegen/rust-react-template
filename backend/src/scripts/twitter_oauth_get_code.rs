extern crate oauth2;

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};

use std::env;

/*

https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code

https://developer.twitter.com/en/docs/authentication/oauth-2-0/user-context-making-requests-on-behalf-of-users
*/

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Set up the config
    let twitter_client_id = ClientId::new(
        env::var("TWITTER_OAUTH_CLIENT_ID").expect("Missing the CLIENT_ID environment variable."),
    );
    let twitter_client_secret = ClientSecret::new(
        env::var("TWITTER_OAUTH_CLIENT_SECRET")
            .expect("Missing the CLIENT_SECRET environment variable."),
    );

    let auth_url = AuthUrl::new("https://twitter.com/i/oauth2/authorize".into()).unwrap();
    let token_url = TokenUrl::new("https://api.twitter.com/2/oauth2/token".into()).unwrap();

    let redirect_url = "http://localhost:8000/api/oauth/twitter/callback";

    // Set up the OAuth2.0 client
    let client = BasicClient::new(
        twitter_client_id,
        Some(twitter_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.into()).expect("Invalid redirect URL"));

    // Generate the PKCE challenge and verifier.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))
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
