


use ethers::{types::Address, utils::to_checksum};

use chrono::{DateTime, NaiveDateTime, Utc};

use degen_sql::db::postgres::postgres_db::Database; 

 
use degen_sql::db::postgres::models::model::PostgresModelError;

use tokio_postgres::Row;
use cron::Schedule;
 
use std::str::FromStr;
use std::time::Duration;


 
 



pub struct Plan{
    pub id:i32,  
    pub prompt_base: String,
    pub ai_model: Option<String> 

}

impl Plan {

    pub fn from_joining_row(row:&Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("plan_id");  
       
        let prompt_base: String = row.get("plan_prompt_base");

        let ai_model: Option<String> = row.try_get("plan_ai_model").ok();  
        

        Ok(Self{
            id,
            prompt_base,
            ai_model   
        })

    }

    pub fn get_select_query() -> String {

        " SELECT 
        plan.id AS plan_id,
        plan.prompt_base AS plan_prompt_base,
        plan.ai_model AS plan_ai_model

         ".into()

    } 

}

pub struct PlansModel {}

impl PlansModel {





/*
    pub async fn insert_one(
       

        custom_context: String,   //add later to the table 
        cron_expression: String, 

        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
         

        let insert_result = psql_db
            .query_one(
                "
            INSERT INTO scheduled_tasks 
            (
            parent_user_id,
            plan_id,
            cron_expression,
                    
            ) 
            VALUES ($1, $2, $3 )
            RETURNING id;
            ",
                &[
                    &parent_user_id,
                    &plan_id,
                    &cron_expression,  
                ],
            )
            .await;

        match insert_result {
            Ok(row) => Ok(row.get(0)), // Successfully inserted new row and got its ID.
            Err(e) => {
                eprintln!("Database error: {:?}", e);

                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    
 
    
*/


}



/*




*/