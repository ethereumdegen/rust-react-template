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
 

pub struct OAuthGetCodeResponse {
    pub auth_url: Url,
    pub code_verifier: String,
    pub state: String,
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