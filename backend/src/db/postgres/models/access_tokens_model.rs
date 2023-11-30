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

//rename me to access token ! 


#[derive(Serialize)]
pub struct AccessToken {
    pub public_address: Address,
    pub parent_client_id: Option<String>,
    pub auth_token: String,
    pub scopes: Vec<String>,
    pub created_at: ChronoDateTime,
    pub expires_at: ChronoDateTime,
}

impl AccessToken {
    fn from_row(row: Row) -> Result<Self, PostgresModelError> {
        let address = row.get::<_, String>("public_address");

        let created_at_utc = row.get("created_at");

        Ok(Self {
            public_address: Address::from_str(&address)
                .map_err(|_e| PostgresModelError::AddressParseError)?,
            auth_token: row.get::<_, String>("token"),
            parent_client_id: row.try_get("parent_client_id").ok(),
            scopes: AuthScopes::new_from_opt_string(  row.try_get::<_, String>("scopes").ok() ).to_vec(),
            created_at: ChronoDateTime::new(created_at_utc),
            expires_at: ChronoDateTime::new(AccessToken::expires_at(created_at_utc)),
        })
    }

    fn expires_at(created_at: DateTime<Utc>) -> DateTime<Utc> {
        created_at + Duration::days(1)
    }
}

pub struct AccessTokensModel {}

impl AccessTokensModel {
    pub async fn create_new_authenticated_user_session(
        public_address: Address,
        psql_db: &Database,
    ) -> Result<
        //public address, session token, expires at
        AccessToken,
        PostgresModelError,
    > {
        //insert a user with this public address if it doesnt exist

        let  insert_user_result = UsersModel::find_or_create_user(
            public_address, 
            None, 
            &psql_db
            ).await;
             
             
      //  let user_roles = user_record.roles;
             
             println!("user   {:?}", insert_user_result );  
        let user_roles = match insert_user_result.ok(){
            Some(usr) => Some(usr.roles),
            None=> None 
        }.unwrap_or_default();
        
        println!("user roles {:?}", user_roles);  
        let scopes =  AuthScopes::get_scopes_from_user_roles( user_roles ).to_vec();
              println!("scopes {:?}", scopes);  
        //if insert_user_result was an error where a user WOULDNT exist, this should error here.

        //insert a user session  !
        AccessTokensModel::insert_new_user_session(
            public_address.clone(), 
            None,  //no client id with a normal user login -- only oauths
            scopes,
            &psql_db
            
            ).await
    }

    pub async fn insert_new_user_session(
        public_address: Address,
        parent_client_id: Option<String>,
        scopes: Vec<String>,
        psql_db: &Database,
    ) -> Result<
        //public address, session token, expires at
        AccessToken,
        PostgresModelError,
    > {
        let _formatted_address = to_checksum(&public_address, None).to_string();

        let session_token = generate_random_uuid();

        let _created_at: DateTime<Utc> = Utc::now() + Duration::days(1);
        
        let scopes_string = scopes.join(",");

        let row = psql_db
            .query_one(
                "
            INSERT INTO access_tokens 
            ( token, public_address , parent_client_id, scopes ) 
            VALUES ( $1, $2, $3, $4 )
             ON CONFLICT(token) DO NOTHING
            RETURNING id, token, scopes, public_address, parent_client_id, created_at;
            ",
                &[&session_token, &DomainEthAddress(public_address), &parent_client_id, &scopes_string],
            )
            .await?;

        // let _created_at:DateTime<Utc> = Utc::now();

        AccessToken::from_row(row)
    }

    pub async fn find_authenticated_user_session(
        auth_token: String,
        psql_db: &Database,
    ) -> Result<
        //public address, session token, expires at
        AccessToken,
        PostgresModelError,
    > {
        let row = psql_db
            .query_one(
                "SELECT 
        id, 
        token, 
        scopes,
        public_address, 
        created_at
         FROM access_tokens 
         WHERE token = ($1)
         LIMIT 1;",
                &[&auth_token],
            )
            .await?;

            AccessToken::from_row(row)
    }
}


