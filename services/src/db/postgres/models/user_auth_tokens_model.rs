


use ethers::{types::Address, utils::to_checksum};

use chrono::{DateTime, NaiveDateTime, Utc};

use degen_sql::db::postgres::postgres_db::Database; 

use serde::{Serialize,Deserialize};

 
use degen_sql::db::postgres::models::model::PostgresModelError;

use tokio_postgres::Row;
use cron::Schedule;
 
use std::str::FromStr;
use std::time::Duration;

use crate::types::openai_completion::OpenAiCompletion;

use super::delivery_method_model::DeliveryMethod;
use super::scheduled_task_model::ScheduledTask;
use super::user_model::User;
use serde_json::Value as JsonValue;

 
  

pub struct UserAuthToken {
    pub id: i32,
    pub parent_user_id: i32,
    pub auth_token: String,
    pub service_name: String // like twitter 
}

impl UserAuthToken {

    pub fn from_joining_row(row: &Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("auth_token_id");
        let parent_user_id:i32 = row.get("auth_token_parent_user_id");
        let auth_token = row.get("auth_token_auth_token");  //lol rename ? 
        let service_name = row.get("auth_token_service_name");
        
        Ok(Self {
            id,
            parent_user_id,
            auth_token,
            service_name
        })
    }

    pub fn get_select_query() -> String {
        "SELECT 
        auth_token.id AS auth_token_id,
        auth_token.parent_user_id AS auth_token_parent_user_id,
        auth_token.auth_token AS auth_token_auth_token,
        auth_token.service_name AS auth_token_service_name
        ".into()
    }
}

pub struct UserAuthTokensModel {}

impl UserAuthTokensModel { 
     

 /*
    pub async fn insert_one(
         parent_scheduled_task_id: i32, 
         body: CompletionOutputBody,
        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
         

        let insert_result = psql_db
            .query_one(
                "
            INSERT INTO completion_outputs 
            (
            parent_scheduled_task_id,
            body 
                    
            ) 
            VALUES ($1, $2 )
            RETURNING id;
            ",
                &[
                    &parent_scheduled_task_id,
                    &serde_json::to_string(&body).unwrap()
                ],
            )
            .await;

        match insert_result {
            Ok(row) => Ok(row.get(0)), // Successfully inserted new row and got its ID.
            Err(e) => {
                eprintln!("Database error: Completion Output: {:?}", e);

                Err(PostgresModelError::Postgres(e))
            }
        }
    }
*/
    
  /*
   pub async fn update_delivered_at(
        completion_output_id: i32, 
        

        psql_db: &Database,
    ) -> Result<(), PostgresModelError> {
         

       let update_result = psql_db
        .execute(
            "
            UPDATE completion_outputs
            SET delivered_at = NOW()
            WHERE id = $1;
            ",
            &[&completion_output_id],
        )
        .await;

    match update_result {
        Ok(_) => Ok(()), // Successfully updated the row.
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(PostgresModelError::Postgres(e))
        }
    } 
    }
*/
    


    pub async fn find_valid_auth_token_for_user(
        user_id: i32, 
        service_name: String,
        psql_db: &Database,
    ) -> Result< Vec<UserAuthToken> , PostgresModelError> {
    
    let where_clause = "".to_string();
      
      
      println!("{}", format!(  "
        {}
        {}
        FROM user_auth_tokens AS auth_token 
        
        WHERE auth_token.parent_user_id = ($1) AND auth_token.service_name = ($2)
        
         ;
        " , 
        UserAuthToken::get_select_query(),
     //   UserAuthToken::get_joins_query(),
        where_clause
       ) ); 
        
        
        let rows = psql_db
            .query(
           format!(  "
        {}
        {}
          
            FROM user_auth_tokens AS auth_token 
            
          WHERE auth_token.parent_user_id = ($1) AND auth_token.service_name = ($2)
        
         ;
        " , 
        UserAuthToken::get_select_query(),
    //    UserAuthToken::get_joins_query(),
        where_clause
       ).as_str(),
                &[
                  
                    &user_id,
                    &service_name
                ],
            )
            .await;
    
      
        
        match rows {
            Ok(rows) => {
                let mut tokens = Vec::new();

                for row in rows {
                    
                    let token = UserAuthToken::from_joining_row(&row)?; 

                    tokens.push(token)
                }

                Ok(tokens)
            }
            Err(e) => {
                eprintln!("Database error: AuthToken {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }
    
    
    

}


 