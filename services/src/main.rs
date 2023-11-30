
use degen_sql::db::postgres::postgres_db::{Database, DatabaseCredentials};
use ethers::types::Address;

use tokio::time::{sleep, Duration};
use std::str::FromStr;
use std::sync::Arc;

use crate::tasks::execute_scheduled_prompt_task::ExecuteScheduledPromptTask;
use crate::tasks::deliver_completion_outputs_task::DeliverCompletionOutputsTask;

/*
use tasks::invoice_status_update_task::InvoiceStatusUpdateTask;
use tasks::read_paid_invoice_events_task::ReadPaidInvoiceEventsTask;
use tasks::price_oracle_task::PriceOracleTask;
*/

extern crate dotenv;


use dotenv::dotenv;

 
mod service_task;
mod tasks;
mod types;
mod db;

 
use crate::service_task::Runnable;
 
 
#[derive(Clone)]
pub struct AppState {
    database: Arc<Database> 
}

#[tokio::main]
async fn main() {
    
    //load env
    env_logger::init();
    dotenv().ok();
    
    let database_credentials = DatabaseCredentials::from_env();
    
    let database = Arc::new(Database::connect(
        database_credentials, None
    ).await.unwrap());
      
    let app_state = Arc::new( AppState {
            database: Arc::clone(&database)  
    });
    
    let scheduled_prompt_task = ExecuteScheduledPromptTask::new(  
        20_000,
        Arc::clone(&app_state)  
    );

    ExecuteScheduledPromptTask::start( scheduled_prompt_task ).await;
    
    let deliver_outputs_task = DeliverCompletionOutputsTask::new(
        20_000,
        Arc::clone(&app_state)
        
    );
    
    DeliverCompletionOutputsTask::start(deliver_outputs_task).await;
    
   

    let running = true;
    
    // Let the tasks run forever 
    while running {
        loop {
            sleep(Duration::from_secs(10)).await;
        }
    }



}
