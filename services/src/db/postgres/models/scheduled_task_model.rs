


use ethers::{types::Address, utils::to_checksum};

use chrono::{DateTime, NaiveDateTime, Utc};

use degen_sql::db::postgres::postgres_db::Database; 

 
use degen_sql::db::postgres::models::model::PostgresModelError;

use tokio_postgres::Row;
use cron::Schedule;
 
use std::str::FromStr;
use std::time::Duration;

use super::plan_model::Plan;
use super::user_model::User;
use super::delivery_method_model::DeliveryMethod;

use crate::types::chronodatetime::ChronoDateTime;


pub struct ScheduledTaskComplex{
   pub scheduled_task: ScheduledTask,
   pub plan: Plan,
   pub user: User,
   pub delivery_method: DeliveryMethod 
}

// finish me 
impl ScheduledTaskComplex {


    fn from_row(row:&Row) -> Result<Self, PostgresModelError> {
        

        Ok(Self{
            scheduled_task : ScheduledTask::from_joining_row(row)?,
            plan : Plan::from_joining_row(row)?,
            user : User::from_joining_row(row)? ,
            delivery_method : DeliveryMethod::from_joining_row(row)?
            

        })

    }


    fn get_select_query() -> String {

        " SELECT 
        scheduled_task.id AS scheduled_task_id,
        scheduled_task.parent_user_id AS scheduled_task_parent_user_id,
        scheduled_task.plan_id AS scheduled_task_plan_id,
        scheduled_task.cron_expression AS scheduled_task_cron_expression,
        scheduled_task.executed_at AS scheduled_task_executed_at,

         plan.id AS plan_id,
        plan.prompt_base AS plan_prompt_base,
        plan.ai_model AS plan_ai_model,
        
        
        user_row.id AS user_id,
        user_row.public_address AS user_public_address,
        

        delivery_method.id AS delivery_method_id,
        delivery_method.name AS delivery_method_name 
      
         
         ".into()

    }

    fn get_joins_query() -> String {

        "FROM scheduled_tasks AS scheduled_task
        INNER JOIN plans AS plan ON scheduled_task.plan_id = plan.id
        INNER JOIN users AS user_row ON scheduled_task.parent_user_id = user_row.id
        
        INNER JOIN delivery_methods AS delivery_method ON scheduled_task.delivery_method_id = delivery_method.id

                 
        ".into()
        
    }


}



pub struct ScheduledTask{
    pub id:i32, 
    pub parent_user_id: i32,
    pub plan_id: i32,
    pub cron_expression: String,
    pub executed_at: DateTime<Utc>

}

impl ScheduledTask {

    pub fn from_joining_row(row:&Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("scheduled_task_id");


        let parent_user_id = row.get::<_,i32>("scheduled_task_parent_user_id");
        let plan_id = row.get::<_,i32>("scheduled_task_plan_id");

        let cron_expression: String = row.get("scheduled_task_cron_expression");
 
        let executed_at = ChronoDateTime::new(row.try_get("scheduled_task_executed_at")?).as_datetime();


        Ok(Self{
            id,
            parent_user_id,
            plan_id,
            cron_expression,
            executed_at 

        })

    }

    fn get_select_query() -> String {

        " SELECT 
        scheduled_task.id AS scheduled_task_id
        scheduled_task.parent_user_id AS scheduled_task_parent_user_id,
        scheduled_task.plan_id AS scheduled_task_plan_id,
        scheduled_task.cron_expression AS scheduled_task_cron_expression,
        scheduled_task.executed_at AS scheduled_task_executed_at


         ".into()

    }

    pub fn is_overdue_to_execute(&self) -> bool {


      
        let cron_expression: String = self.cron_expression.clone();
     
        let executed_at = &self.executed_at;

        println!("cron exp is {}", cron_expression.clone());
    
        // Parse the cron expression using the `cron` crate
        let cron_schedule = match Schedule::from_str(&cron_expression){
            Ok(sched) => sched,
            Err(e) => {
                println!("WARN: Could not parse cron of task {} {:?}! ", self.id, e);
                return false 
            } 

        };
    
        // Get the next time the task is supposed to run after its last execution time
        let next_time = cron_schedule.after(&executed_at).next().unwrap();
    
        // Check if the task should run now
        let now = Utc::now();
        if next_time <= now {
            
            
            return true 
        }
    
       return false 
    }
    

}

pub struct ScheduledTasksModel {}

impl ScheduledTasksModel {
    pub async fn insert_one(
        parent_user_id: i32, 
        plan_id: i32,

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

    
   pub async fn update_executed_at(
        scheduled_task_id: i32, 
        

        psql_db: &Database,
    ) -> Result<(), PostgresModelError> {
         

       let update_result = psql_db
        .execute(
            "
            UPDATE scheduled_tasks
            SET executed_at = NOW()
            WHERE id = $1;
            ",
            &[&scheduled_task_id],
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

    
    

    pub async fn find_all_scheduled_tasks(
       
        limit: u32,
        psql_db: &Database,
    ) -> Result<Vec<ScheduledTask>, PostgresModelError> {
 

        let rows = psql_db
            .query(
           format!(  "
        {}

        FROM scheduled_tasks AS scheduled_task
        WHERE executed_at <= (NOW() - INTERVAL '8 hours')
        LIMIT $1;
        " , ScheduledTask::get_select_query()  ).as_str(),
                &[
                  
                    &(limit as i64),
                ],
            )
            .await;

        match rows {
            Ok(rows) => {
                let mut tasks = Vec::new();

                for row in rows {
                    
                    let task = ScheduledTask::from_joining_row(&row)?; 

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
    
    
    


    pub async fn find_all_scheduled_tasks_complex(
       
        limit: u32,
        psql_db: &Database,
    ) -> Result<Vec<ScheduledTaskComplex>, PostgresModelError> {
 

      println!("{}", format!(  "
        {}
        {}
        WHERE executed_at <= (NOW() - INTERVAL '8 hours')
        LIMIT $1;
        " , 
        ScheduledTaskComplex::get_select_query(),
        ScheduledTaskComplex::get_joins_query() 
       ) ); 
        
        
        let rows = psql_db
            .query(
           format!(  "
        {}
        {}
        WHERE executed_at <= (NOW() - INTERVAL '8 hours')
        LIMIT $1;
        " , 
        ScheduledTaskComplex::get_select_query(),
        ScheduledTaskComplex::get_joins_query() 
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
                    
                    let task = ScheduledTaskComplex::from_row(&row)?; 

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



/*




*/