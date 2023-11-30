/*

POST https://{shop}.myshopify.com/admin/oauth/access_token?client_id={client_id}&client_secret={client_secret}&code={authorization_code}

*/

/*

https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code

https://developer.twitter.com/en/docs/authentication/oauth-2-0/user-context-making-requests-on-behalf-of-users
*/

use dotenvy::dotenv;
use serde::Deserialize;

use std::env;

#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub scope: String,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Set up the config

    // Set these variables with the appropriate values
    let shop = "quickstart-540f2e88";
    let client_id: String = env::var("SHOPIFY_OAUTH_CLIENT_ID").unwrap().into();
    let client_secret: String = env::var("SHOPIFY_OAUTH_CLIENT_SECRET").unwrap().into();

    let authorization_code: String = "4cb78b02de9ba5b1606a9d79ed7cd7d7".into();

    // Create the URL
    let url = format!(
        "https://{}.myshopify.com/admin/oauth/access_token?client_id={}&client_secret={}&code={}",
        shop, client_id, client_secret, authorization_code
    );

    // Execute POST request
    let client = reqwest::Client::new();
    let response_result = client.post(&url).send().await;

    // println!("res {:?}", response_result);

    if let Ok(response) = response_result {
        let token_info: TokenResponse = response.json().await.expect("FAIL");

        println!("body {:?}", token_info);
    }
}

//https://github.com/bk-rs/oauth2-rs/blob/main/oauth2-signin/src/web_app/signin_flow.rs
