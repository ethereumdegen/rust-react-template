extern crate oauth2;

use reqwest;
use std::env;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    // add any other fields that Twitter's OAuth2 response contains
}

/*

https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code

https://developer.twitter.com/en/docs/authentication/oauth-2-0/user-context-making-requests-on-behalf-of-users
*/

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Set up the config
    let twitter_client_id =
        env::var("TWITTER_OAUTH_CLIENT_ID").expect("Missing the CLIENT_ID environment variable."); //ClientId::new(env::var("TWITTER_OAUTH_CLIENT_ID").expect("Missing the CLIENT_ID environment variable."));

    let twitter_client_secret = env::var("TWITTER_OAUTH_CLIENT_SECRET")
        .expect("Missing the CLIENT_SECRET environment variable.");

    //  let token_url = TokenUrl::new( "https://api.twitter.com/2/oauth2/token" .into()).unwrap();

    let redirect_url = "http://localhost:8000/api/oauth/twitter/callback";

    //these come from the get_code process!! the verifier is the secret generated and the code is from twitters servers and via the callback

    let code_verifier: String = "1Fj6AhtI4ve-y9UIULgCVQCdymaPT8R4XgW3dlm5XBc".into();
    let code:String = "ZVZteTRoeGltc1pjaUpfX2JGcFFmbEpELWNCeU5ZeUZjbmtfd05vZHdNOFgyOjE2OTc5MTY2NTY1MzY6MTowOmFjOjE".to_string();

    //https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code

    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("code", code.to_string());
    payload.insert("redirect_uri", redirect_url.to_string());
    payload.insert("grant_type", "authorization_code".to_string());
    payload.insert("code_verifier", code_verifier.to_string());

    let encoded_auth = base64::encode(format!("{}:{}", twitter_client_id, twitter_client_secret));
    let auth_payload = Box::leak(format!("Basic {}", encoded_auth).into_boxed_str());

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );
    headers.insert(AUTHORIZATION, HeaderValue::from_str(auth_payload).unwrap());

    let res = client
        .post("https://api.twitter.com/2/oauth2/token")
        .headers(headers)
        .form(&payload)
        .send()
        .await
        .expect("REQUEST FAILED");

    println!("got res {:?}", res);

    let token_info: TokenResponse = res.json().await.expect("FAIL");
    println!("Access Token: {:?}", token_info);
}

//https://github.com/bk-rs/oauth2-rs/blob/main/oauth2-signin/src/web_app/signin_flow.rs
