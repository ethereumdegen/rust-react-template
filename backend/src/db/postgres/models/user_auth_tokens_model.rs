use std::str::FromStr;

use tokio_postgres::{Row};

use super::super::postgres_db::Database;
use super:: PostgresModelError;

//use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use chrono::{DateTime, Duration, Utc};

use crate::{db::postgres::domains::eth_address::DomainEthAddress, util::auth_scopes::AuthScopes};
use crate::types::chronodatetime::ChronoDateTime;
use crate::util::rand::generate_random_uuid;

use serde::{Serialize};

use ethers::types::Address;
use ethers::utils::to_checksum;

use super::users_model::{UsersModel };
use crate::db::postgres::models::{expand_join_query_rows,JoinQueryRow}; 

 

#[derive(Serialize)]
pub struct UserAuthToken {
    pub id: i32,
    pub parent_user_id: i32,
    pub parent_client_id: String,
    pub code: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub expires_at: Option<ChronoDateTime>,
    pub created_at: ChronoDateTime,
}

impl UserAuthToken {
    pub fn from_joining_row(row: &Row) -> Result<Self, PostgresModelError> {
 
 
        Ok(Self {
            id: row.try_get("auth_code_id")?,
            parent_user_id: row.get("auth_code_parent_user_id"),
            parent_client_id: row.get("auth_code_parent_client_id"),
            code: row.get("auth_code"),
            scopes:  AuthScopes::new_from_opt_string( row.try_get("auth_code_scopes").ok() ).to_vec(), 
            redirect_uri: row.get("auth_code_redirect_uri"),
            expires_at: row.try_get("auth_code_expires_at").ok().map(|time| ChronoDateTime::new(time)),
            created_at: ChronoDateTime::new(row.get("auth_code_created_at")),
        })
    }

    pub fn get_query_rows() -> Vec<JoinQueryRow> {
        vec![
            JoinQueryRow::new("oauth_authorization_code.id", "auth_code_id"),
            JoinQueryRow::new("oauth_authorization_code.parent_user_id", "auth_code_parent_user_id"),
            JoinQueryRow::new("oauth_authorization_code.parent_client_id", "auth_code_parent_client_id"),
            JoinQueryRow::new("oauth_authorization_code.code", "auth_code"),
            JoinQueryRow::new("oauth_authorization_code.scopes", "auth_code_scopes"),
            JoinQueryRow::new("oauth_authorization_code.redirect_uri", "auth_code_redirect_uri"),
            JoinQueryRow::new("oauth_authorization_code.expires_at", "auth_code_expires_at"),
            JoinQueryRow::new("oauth_authorization_code.created_at", "auth_code_created_at"),
        ]
    }

    pub fn get_select_query() -> String {
        let query_row_expanded: String = expand_join_query_rows(Self::get_query_rows());
        format!("SELECT {} FROM oauth_authorization_codes AS oauth_authorization_code", query_row_expanded)
    }
}


pub struct UserAuthTokensModel {}

impl UserAuthTokensModel {
    
     
    
    pub async fn insert_one(
        parent_user_id: i32,
        code:  String ,
        client_id:  String ,
        redirect_uri:  String ,
        scopes: Vec<String>, 
          
        psql_db: &Database,
    ) -> Result<
        //public address, session token, expires at
        i32,
        PostgresModelError,
    > {

        let scopes_raw = scopes.join(",");
        
        let expires_at = ChronoDateTime::new( Utc::now() + Duration::minutes(5) );
        
        let row = psql_db
        .query_one(
            "
        INSERT INTO oauth_authorization_codes 
        ( parent_user_id, parent_client_id,  code, redirect_uri, scopes , expires_at) 
        VALUES ( $1, $2, $3, $4, $5, $6 )
         ON CONFLICT(code) DO NOTHING
        RETURNING id, parent_user_id, parent_client_id, code, redirect_uri, scopes, created_at;
        ",
            &[&parent_user_id, &client_id, &code, &redirect_uri,  &scopes_raw, &expires_at.as_datetime()],
        )
        .await?;

        
        Ok(  row.get(0) )
    }
     
     
     
     pub async fn find_by_code( 
        code: &String, 
        psql_db: &Database,
     ) ->  Result<  
        OAuthAuthorizationCode,
        PostgresModelError > {
            
            
              let row = psql_db
            .query_one(
                format!("  
                         {}
                         
                        WHERE oauth_authorization_code.code = $1 
                        LIMIT 1;
                        " ,OAuthAuthorizationCode::get_select_query()
                        ).as_str(),
                &[&code],
            )
            .await?;

        let shop = OAuthAuthorizationCode::from_joining_row(&row)?;

        Ok(shop)
        
    }

}