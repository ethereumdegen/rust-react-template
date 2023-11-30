/*

https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code

https://developer.twitter.com/en/docs/authentication/oauth-2-0/user-context-making-requests-on-behalf-of-users
*/

use dotenvy::dotenv;
use tempo_backend::util::oauth::shopify_oauth_get_code;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Set up the config

    let auth_url = shopify_oauth_get_code().unwrap();
    println!("Open this URL in your browser:\n{}\n", auth_url);

    // let code = AuthorizationCode::new("some authorization code".to_string()); // This should be provided by your OAuth2 provider.
}

//https://github.com/bk-rs/oauth2-rs/blob/main/oauth2-signin/src/web_app/signin_flow.rs
