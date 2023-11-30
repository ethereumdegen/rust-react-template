extern crate oauth2;

use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl,
};
//use oauth2_twitter as twitter;
use reqwest::Url;
use std::collections::HashMap;
use std::env;

use super::backend_server_error::BackendServerError;

/*
#[derive(thiserror::Error, Debug)]
pub enum OAuthError {
    #[error("Invalid client ID")]
    InvalidClientID,

    // Add more variants here as needed
}
*/

/*

WHen i go to install my app into shopify,

shopify redirect the user to here

http://localhost:8000/api/oauth/shopify/callback?hmac=6d349db4b25793507b8d0284cf4b47843454bd9a01c90aa22ea7f4d3eec950ed&host=YWRtaW4uc2hvcGlmeS5jb20vc3RvcmUvcXVpY2tzdGFydC01NDBmMmU4OA&shop=quickstart-540f2e88.myshopify.com&timestamp=1698197853







*/

pub struct ShopifyAuthUrlBuilder {
    shop: String,
    client_id: String,
    scopes: String,
    redirect_uri: String,
    nonce: String,
}

impl ShopifyAuthUrlBuilder {
    fn to_string(&self) -> String {
        format!(" https://{}.myshopify.com/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state={}",            
             
             self.shop,
             self.client_id,
             self.scopes,
             self.redirect_uri,
             self.nonce 
             )
    }
}

//https://shopify.dev/docs/apps/auth/oauth/getting-started

pub fn shopify_oauth_get_code() -> Result<String, BackendServerError> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    println!("oauth1");
    // Set up the config

    //whhy cant i do dotenv !?

    let oauth_client_id = ClientId::new(
        env::var("SHOPIFY_OAUTH_CLIENT_ID").map_err(|_e| BackendServerError::ServerConfigError)?,
    );

    println!("oauth2");

    /* let oauth_client_secret = ClientSecret::new(env::var("SHOPIFY_OAUTH_CLIENT_SECRET")
        .map_err(|e| BackendServerError::ServerConfigError)?);
    */
    println!("oauth");

    //    let token_url = TokenUrl::new( "https://api.twitter.com/2/oauth2/token" .into())
    //  .map_err(|e| BackendServerError::ServerConfigError)?;

    let redirect_uri: String = "http://localhost:8000/api/oauth/shopify/callback".into();

    //  https://{shop}.myshopify.com/admin/oauth/authorize?client_id={client_id}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}

    let auth_url_builder = ShopifyAuthUrlBuilder {
        shop: "quickstart-540f2e88".into(),
        client_id: oauth_client_id.clone().into(),
        scopes: "read_orders,read_shipping".into(),
        redirect_uri: redirect_uri.to_string(),
        nonce: "123456".into(),
    };

    let auth_url = auth_url_builder.to_string();

    /*
        let client = BasicClient::new(
            oauth_client_id,
            Some(oauth_client_secret),
            auth_url,
           None  // Some(token_url),
        )
        .set_redirect_uri(
            redirect_uri
        );

        // Generate the PKCE challenge and verifier.
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();



        // Generate the authorization URL to which we'll redirect the user.
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("tweet.read".to_string()))
            .add_scope(Scope::new("tweet.write".to_string()))
            .add_scope(Scope::new("users.read".to_string()))

            .add_scope(Scope::new("offline.access".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();
    */
    println!("Open this URL in your browser:\n{}\n", auth_url);

    // need this to swap for auth token later . prob need to store in db or something.
    //println!("code verifier secret is {}  . You will need this too for the next step.", pkce_verifier.secret());

    /*

    https://example.org/some/redirect/uri?code={authorization_code}&hmac=da9d83c171400a41f8db91a950508985&host={base64_encoded_hostname}&shop={shop_origin}&state={nonce}&timestamp=1409617544

    */

    //  POST https://{shop}.myshopify.com/admin/oauth/access_token?client_id={client_id}&client_secret={client_secret}&code={authorization_code}

    //store the code verifier secret in the db !!

    Ok(auth_url)
}

pub struct OAuthGetCodeResponse {
    pub auth_url: Url,
    pub code_verifier: String,
    pub state: String,
}

pub async fn indiefuture_oauth_get_code() -> Result<OAuthGetCodeResponse, BackendServerError> {
    let indiefuture_frontend_domain = "https://indiefuture.com";
    let indiefuture_backend_domain = "https://api.indiefuture.com";

    let _tempova_backend_domain = "https://api.tempova.com";

    let oauth2_auth_url = format!("{}/oauth2/authorize", indiefuture_frontend_domain);

    let oauth2_token_url = format!("{}/api/v1/oauth2/token", indiefuture_backend_domain);

    let tempova_backend_domain = "https://api.tempova.com";

    let redirect_url = format!("{}/api/oauth2/indiefuture/callback", tempova_backend_domain);

    return oauth_get_code(
        ClientId::new(
            env::var("INDIEFUTURE_OAUTH_CLIENT_ID")
                .map_err(|_e| BackendServerError::ServerConfigError)?,
        ),
        ClientSecret::new(
            env::var("INDIEFUTURE_OAUTH_CLIENT_SECRET")
                .map_err(|_e| BackendServerError::ServerConfigError)?,
        ),
        AuthUrl::new(oauth2_auth_url.into()).map_err(|_e| BackendServerError::ServerConfigError)?,
        TokenUrl::new(oauth2_token_url.into())
            .map_err(|_e| BackendServerError::ServerConfigError)?,
        RedirectUrl::new(redirect_url.into())
            .map_err(|_e| BackendServerError::ServerConfigError)?,
        vec![Scope::new("tweet.read".to_string())],
    )
    .await;
}


pub async fn google_oauth_get_code() -> Result<OAuthGetCodeResponse, BackendServerError> {
    
    let oauth2_auth_url = "https://accounts.google.com/o/oauth2/auth".to_string();

    let oauth2_token_url = "https://oauth2.googleapis.com/token".to_string();

    //let tempova_backend_domain = "https://api.tempova.com";
    let tempova_backend_domain = "http://localhost:9000";

    let redirect_url = format!("{}/api/oauth2/google/callback", tempova_backend_domain);

    return oauth_get_code(
        ClientId::new(
            env::var("GOOGLE_OAUTH_CLIENT_ID")
                .map_err(|_e| BackendServerError::ServerConfigError)?,
        ),
        ClientSecret::new(
            env::var("GOOGLE_OAUTH_CLIENT_SECRET")
                .map_err(|_e| BackendServerError::ServerConfigError)?,
        ),
        AuthUrl::new(oauth2_auth_url.into()).map_err(|_e| BackendServerError::ServerConfigError)?,
        TokenUrl::new(oauth2_token_url.into())
            .map_err(|_e| BackendServerError::ServerConfigError)?,
        RedirectUrl::new(redirect_url.into())
            .map_err(|_e| BackendServerError::ServerConfigError)?,
       
       
            vec![
                
            
            Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
            Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string())
            
            
            ],
    )
    .await;
}

pub async fn oauth_get_code(
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    scopes: Vec<Scope>,
) -> Result<OAuthGetCodeResponse, BackendServerError> {
    // Set up the OAuth2.0 client
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url);

    // Generate the PKCE challenge and verifier.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let mut client = client.authorize_url(CsrfToken::new_random);

    for scope in scopes {
        client = client.add_scope(scope);
    }
    /*

    client
        .authorize_url(CsrfToken::new_random)


        .add_scope(Scope::new("tweet.read".to_string()))
        .add_scope(Scope::new("tweet.write".to_string()))
        .add_scope(Scope::new("users.read".to_string()))

        .add_scope(Scope::new("offline.access".to_string()))

    */

    // Generate the authorization URL to which we'll redirect the user.
    let (auth_url, _csrf_token) = client.set_pkce_challenge(pkce_challenge).url();

    let auth_url_string = auth_url.to_string().clone();
    let parsed_auth_url = Url::parse(&auth_url_string).unwrap();
    let query_map: HashMap<String, String> = parsed_auth_url.query_pairs().into_owned().collect();

    let state = query_map.get("state").unwrap();

    //println!("Open this URL in your browser:\n{}\n", auth_url);

    // need this to swap for auth token later . prob need to store in db or something.
    // println!("code verifier secret is {}  . You will need this too for the next step.", pkce_verifier.secret());

    //store the code verifier secret in the db !!

    Ok(OAuthGetCodeResponse {
        auth_url,
        code_verifier: pkce_verifier.secret().clone(),
        state: state.clone(),
    })

    // let code = AuthorizationCode::new("some authorization code".to_string()); // This should be provided by your OAuth2 provider.
}

pub async fn twitter_oauth_get_code() -> Result<OAuthGetCodeResponse, BackendServerError> {
    // Load environment variables from .env file
    // dotenvy::dotenv().ok();

    // Set up the config
    let twitter_client_id = ClientId::new(
        env::var("TWITTER_OAUTH_CLIENT_ID").map_err(|_e| BackendServerError::ServerConfigError)?,
    );
    let twitter_client_secret = ClientSecret::new(
        env::var("TWITTER_OAUTH_CLIENT_SECRET")
            .map_err(|_e| BackendServerError::ServerConfigError)?,
    );

    let auth_url = AuthUrl::new("https://twitter.com/i/oauth2/authorize".into())
        .map_err(|_e| BackendServerError::ServerConfigError)?;
    let token_url = TokenUrl::new("https://api.twitter.com/2/oauth2/token".into())
        .map_err(|_e| BackendServerError::ServerConfigError)?;

    let redirect_url = RedirectUrl::new("http://localhost:8000/api/oauth/twitter/callback".into())
        .map_err(|_e| BackendServerError::ServerConfigError)?;

    return oauth_get_code(
        twitter_client_id,
        twitter_client_secret,
        auth_url,
        token_url,
        redirect_url,
        vec![Scope::new("tweet.read".to_string())],
    )
    .await;

    // Set up the OAuth2.0 client
    /*  let client = BasicClient::new(
         twitter_client_id,
         Some(twitter_client_secret),
         auth_url,
         Some(token_url),
     )
     .set_redirect_uri(
         redirect_url
     );

     // Generate the PKCE challenge and verifier.
     let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();



     // Generate the authorization URL to which we'll redirect the user.
     let (auth_url, csrf_token) = client
         .authorize_url(CsrfToken::new_random)
         .add_scope(Scope::new("tweet.read".to_string()))
         .add_scope(Scope::new("tweet.write".to_string()))
         .add_scope(Scope::new("users.read".to_string()))

         .add_scope(Scope::new("offline.access".to_string()))
         .set_pkce_challenge(pkce_challenge)
         .url();

     println!("Open this URL in your browser:\n{}\n", auth_url);


     // need this to swap for auth token later . prob need to store in db or something.
     println!("code verifier secret is {}  . You will need this too for the next step.", pkce_verifier.secret());


     //store the code verifier secret in the db !!


     Ok((auth_url,pkce_verifier.secret().clone()))
    // let code = AuthorizationCode::new("some authorization code".to_string()); // This should be provided by your OAuth2 provider.
     */
}

//step 2
pub async fn twitter_oauth_get_token() {}
