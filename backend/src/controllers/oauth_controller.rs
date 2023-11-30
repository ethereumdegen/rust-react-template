use crate::util::oauth::{
    
    google_oauth_get_code,

};
use crate::AppState;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::env;

use super::web_controller::WebController;

use actix_web::web::{self, Data, Json, ServiceConfig};

use actix_web::{http::header, HttpResponse};

use tempo_backend::util::backend_server_error::BackendServerError;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

use url::Url;

use crate::db::postgres::models::oauth_requests_model::OAuthRequestsModel;

use oauth2::TokenUrl;


pub struct OAuthController {}

impl OAuthController {}

//POST http://localhost:8000/api/order/create

impl WebController for OAuthController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api/oauth2")
               
                
                .route("/google/init", web::get().to(google_oauth_init))
                .route(
                    "/google/callback",
                    web::get().to(google_oauth_callback),
                )
                
                
               
        );
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    // add any other fields that Twitter's OAuth2 response contains
}

/*
#[derive(Deserialize)]
pub struct ShopifyOAuthInitInput {
         /*  http://localhost:8000/api/oauth/shopify/callback?code=4bc0eae20d0d3ef18d680bb99c9b815c&
         hmac=2c38e3bd15b638075dd517de4d7e3ebb7bfbfadb71a3a5f357324ffb9fade4ce
         &host=YWRtaW4uc2hvcGlmeS5jb20vc3RvcmUvcXVpY2tzdGFydC01NDBmMmU4OA&
         shop=quickstart-540f2e88.myshopify.com&
         state=123456&timestamp=1698199392 */

        pub code: Option<String>,
        pub state: Option<i32> ,

        pub hmac :String,
        pub host :String ,
        pub shop : String ,
        pub timestamp: i32


  }




async fn shopify_oauth_init(
    params: web::Query<ShopifyOAuthInitInput>,
    app_state: Data<AppState>,
) -> actix_web::Result< Json<( )> > {
    let db = &app_state.database;



    println!("hmac is {}", &params.hmac);

     println!("code is {:?}", &params.code);

    Ok(Json(  () ))
}
 */

#[derive(Deserialize)]
pub struct GetOAuthInitInput {
    // pub state: String,
    // pub code: String
}
 

#[derive(Deserialize)]
pub struct GetOAuthCallbackInput {
    pub state: String,
    pub code: String,
}
 

async fn google_oauth_init(
    _params: web::Query<GetOAuthInitInput>,
    app_state: Data<AppState>,
) -> actix_web::Result<Json<String>> {
    let db = &app_state.database;

    //let state = generate_random_alphanumeric(16);
    let oauth_domain = "google".to_string();

    //we can just generate a random STATE and store it in db along w the stuff .  We use STATE as our tether. s

    let get_code_results = google_oauth_get_code(
        //user id goes here ?
    )
    .await?;

    let get_code_url = get_code_results.auth_url;
    let get_code_verifier_secret = get_code_results.code_verifier;

    let state = get_code_results.state;

    // store a database record with the STATE and the get_code_verifier_secret

    let _insert = OAuthRequestsModel::insert_one(state, get_code_verifier_secret, oauth_domain, db)
        .await
        .map_err(|_e| BackendServerError::UnknownError)?;

    //should make something like this
    /*
    http://localhost:8080/oauth2/authorize?response_type=code&client_id=WG4xR0ZnTXU4ZS1JZTZjbGctaWk6MTpjaQ&state=NEAeeH37rp2FSmh7qUPlkA&code_challenge=eSj2sOuNGZEyfoPJG-90X0fD9TQLzcvVXZC4a2Jzt68&code_challenge_method=S256&redirect_uri=http%3A%2F%2Flocalhost%3A9000%2Fapi%2Foauth%2Findiefuture%2Fcallback&scope=offline.access

        */
    Ok(Json(get_code_url.to_string()))
}
 


//gets the oauth token and stores in localstorage or whatev
async fn google_oauth_callback(
    oauth_callback_input: web::Query<GetOAuthCallbackInput>,
    app_state: Data<AppState>,
) -> actix_web::Result<HttpResponse> {
    let db = &app_state.database;

    //we should make sure the state matches what we sent to prevent cross script forging
    //we should store the user auth token in the db ! we can use it now to tweet for them .

    let code = oauth_callback_input.code.clone();
    let state = oauth_callback_input.state.clone();
  //  let _oauth_domain = "google".to_string();

    let oauth_request = OAuthRequestsModel::find_by_state(state, db)
        .await
        .map_err(|_e| BackendServerError::UnknownError)?;

    println!(
        "oauth recvd: {} {} ",
        &oauth_callback_input.code, &oauth_callback_input.state
    );

    //maybe using the STATE i can load this other stuff ?? which was stored by INIT ?
    let code_verifier: String = oauth_request.code_verifier;
    //  let redirect_uri:String = "https://indiefuture.com".into();

    //need to send the state and the client secret to  /token

    //use reqwest  POST !
    // I should have in my request: code, grant_type, client_id and redirect_uri, and the code_verifier.


 

    let oauth_client_id = env::var("GOOGLE_OAUTH_CLIENT_ID")
        .expect("Missing the CLIENT_ID environment variable."); //ClientId::new(env::var("TWITTER_OAUTH_CLIENT_ID").expect("Missing the CLIENT_ID environment variable."));

    let oauth_client_secret = env::var("GOOGLE_OAUTH_CLIENT_SECRET")
        .expect("Missing the CLIENT_SECRET environment variable.");

     let redirect_uri = format!("http://localhost:9000/api/oauth2/google/callback");


     let encoded_auth = base64::encode(format!("{}:{}", oauth_client_id, oauth_client_secret));
    let auth_payload = format!("Basic {}", encoded_auth);

  

    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("code", code.to_string());
    payload.insert("redirect_uri", redirect_uri.to_string());
 //   payload.insert("client_id", oauth_client_id.to_string());
 //   payload.insert("client_secret", oauth_client_secret.to_string());

    payload.insert("code_verifier", code_verifier.to_string());
    payload.insert("grant_type", "authorization_code".to_string());

   // let encoded_auth = base64::encode(format!("{}:{}", oauth_client_id, oauth_client_secret));
  //  let auth_payload = Box::leak(format!("Basic {}", encoded_auth).into_boxed_str());

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_payload).unwrap());

   
   //https://developers.google.com/identity/protocols/oauth2


    let oauth2_token_url = "https://oauth2.googleapis.com/token".to_string() ;


    let res = client
        .post(oauth2_token_url)
        .headers(headers)
        .form(&payload)
        .send()
        .await
        .expect("REQUEST FAILED");

    println!("got res {:?}", res);


  //  let response_body = res.text().await?;
  //      println!("Response Body: {:?}", response_body);

    let access_token: TokenResponse = res.json().await.expect("FAIL");
    println!("Access Token: {:?}", access_token);
    //now we have the full access token so we direct user somewhere cool ? with the full token ?

    //let tempova_frontend_domain = "https://tempova.com";
    let tempova_frontend_domain = "http://localhost:8080";
    let oauth_complete_url = format!("{}/oauth/complete", tempova_frontend_domain);

    let mut final_redirect_uri = Url::parse(&oauth_complete_url).unwrap();

    final_redirect_uri
        .query_pairs_mut()
        .append_pair("access_token", &access_token.access_token)
        .append_pair("access_token_domain", &"indiefuture");

    // After you're done with your logic, to issue a redirect:
    Ok(HttpResponse::Found()
        .header(header::LOCATION, final_redirect_uri.to_string())
        .finish())
}
