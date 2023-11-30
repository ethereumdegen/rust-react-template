


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


 
 //from chat gpt !

 pub type CompletionOutputBody = OpenAiCompletion;



//cant easily join this w user auth token so will need to do a second  db call for that  . 
pub struct CompletionOutputComplex {
    
    pub completion_output: CompletionOutput,
    pub scheduled_task: ScheduledTask,
    pub delivery_method: DeliveryMethod,
    
    pub user : User 
}

impl CompletionOutputComplex{
    
 fn from_row(row:&Row) -> Result<Self, PostgresModelError> {
        

        Ok(Self{
            completion_output: CompletionOutput::from_joining_row(row)? ,
            scheduled_task : ScheduledTask::from_joining_row(row)?, 
            user : User::from_joining_row(row)? ,
            delivery_method : DeliveryMethod::from_joining_row(row) ?          

        })

    }


    fn get_select_query() -> String {

        " SELECT 
        completion_output.id AS completion_output_id,
        completion_output.body AS completion_output_body,
        
        
        scheduled_task.id AS scheduled_task_id,
        scheduled_task.parent_user_id AS scheduled_task_parent_user_id,
        scheduled_task.plan_id AS scheduled_task_plan_id,
        scheduled_task.cron_expression AS scheduled_task_cron_expression,
        scheduled_task.executed_at AS scheduled_task_executed_at,

        
        user_row.id AS user_id,
        user_row.public_address AS user_public_address,
        

        delivery_method.id AS delivery_method_id,
        delivery_method.name AS delivery_method_name 
      
         
         ".into()

    }

    fn get_joins_query() -> String {

        "FROM completion_outputs AS completion_output
        
        INNER JOIN scheduled_tasks AS scheduled_task ON completion_output.parent_scheduled_task_id = scheduled_task.id
        INNER JOIN users AS user_row ON scheduled_task.parent_user_id = user_row.id        
        INNER JOIN delivery_methods AS delivery_method ON scheduled_task.delivery_method_id = delivery_method.id

                 
        ".into()
        
    }

        
}


pub struct CompletionOutput {
    pub id: i32,
    pub body: CompletionOutputBody
}

impl CompletionOutput {

    pub fn from_joining_row(row: &Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("completion_output_id");
        let body: CompletionOutputBody = serde_json::from_str(  row.get::<_, String>("completion_output_body").as_str() ).map_err(|e|PostgresModelError::RowParseError)? ;

        Ok(Self {
            id,
            body
        })
    }

    pub fn get_select_query() -> String {
        "SELECT 
        completion_output.id AS delivery_method_id,
        completion_output.body AS delivery_method_body
        ".into()
    }
}

pub struct CompletionOutputsModel {}

impl CompletionOutputsModel { 
    
    // find all undelivered .. 

 
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

    


    pub async fn find_all_completion_outputs_complex(
        has_been_delivered: bool, 
        limit: u32,
        psql_db: &Database,
    ) -> Result<Vec<CompletionOutputComplex>, PostgresModelError> {
    
    let where_clause = match has_been_delivered {
        true => "
        WHERE completion_output.delivered_at IS NOT NULL
         ".to_string(), 
        false => "
        WHERE completion_output.delivered_at IS NULL
         ".to_string()
    };

      println!("{}", format!(  "
        {}
        {}
        {}
        LIMIT $1;
        " , 
        CompletionOutputComplex::get_select_query(),
        CompletionOutputComplex::get_joins_query(),
        where_clause
       ) ); 
        
        
        let rows = psql_db
            .query(
           format!(  "
        {}
        {}
        {}
        LIMIT $1;
        " , 
        CompletionOutputComplex::get_select_query(),
        CompletionOutputComplex::get_joins_query(),
        where_clause
       ).as_str(),
                &[
                  
                    &(limit as i64),
                ],
            )
            .await;

        match rows {
            Ok(rows) => {
                let mut tasks = Vec::new();

                for row in rows {
                    
                    let task = CompletionOutputComplex::from_row(&row)?; 

                    tasks.push(task)
                }

                Ok(tasks)
            }
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }
    
    
    

}


 