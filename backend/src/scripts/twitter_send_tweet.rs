


    

extern crate oauth2;
extern crate oauth2_twitter;

use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};
use oauth2_twitter as twitter;
use std::env;
use reqwest;

use std::collections::HashMap;
use serde::{Serialize,Deserialize};

use reqwest::header::{HeaderValue,AUTHORIZATION,CONTENT_TYPE,HeaderMap };


#[derive(Serialize, Deserialize,Debug)]
struct TweetResponse {
    pub id: String,
   
    // add any other fields that Twitter's OAuth2 response contains
}


/*

THIS WORKS 
      
curl --location --request POST 'https://api.twitter.com/2/tweets' \
--header 'Authorization: Bearer U21reDJNZkNoa1FzSUItaUJhSkZGX3pjM2FWbGswRmRGalV3U0NFd3ZDVlliOjE2OTc4MzA1ODkyNjE6MToxOmF0OjE' \
--header 'Content-Type: application/json' \
--data '{"text": gm"}'
    

*/
 
#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

   
   let action_url:String =   "https://api.twitter.com/2/tweets" .into() ;


   let auth_token:String = "U21reDJNZkNoa1FzSUItaUJhSkZGX3pjM2FWbGswRmRGalV3U0NFd3ZDVlliOjE2OTc4MzA1ODkyNjE6MToxOmF0OjE".to_string();

    //https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code


    let client = reqwest::Client::new();
    let mut payload = HashMap::new(); 
    payload.insert("text", "gm");
    
   // let encoded_auth = base64::encode(format!("{}:{}", twitter_client_id, twitter_client_secret));
    let auth_payload = Box::leak(format!("Bearer {}", auth_token).into_boxed_str());

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str( auth_payload ).unwrap());


    let res = client
        .post(action_url.to_string())
        .headers(headers)
        .json(&payload)

        .send()
        .await.expect("REQUEST FAILED");

    println!("got res {:?}", res);

    let token_info: TweetResponse = res.json().await.expect("FAIL");
    println!("TweetResponse: {:?}", token_info);
}


//https://github.com/bk-rs/oauth2-rs/blob/main/oauth2-signin/src/web_app/signin_flow.rs