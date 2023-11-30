/*

This task will execute a scheduled prompt.

It will hit Chat Gpt with a combined query of :

1. the prompt in the associated 'plan' that was scheduled 
2. user-specific context data for personalization


The record will then be saved in postgres and somehow given to the user (email?app?text?)



https://platform.openai.com/docs/guides/gpt/chat-completions-api

*/


use async_trait::async_trait;

use crate::{Runnable, service_task::ServiceInterval,
     db::postgres::models::completion_output_model::{CompletionOutputBody, 
         CompletionOutputsModel}};
use serde::{Serialize,Deserialize};
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use std::env;

use degen_sql::db::postgres::postgres_db::Database;
use crate::tasks::ServiceTaskError;
use crate::AppState;

use std::sync::Arc;


use crate::db::postgres::models::scheduled_task_model::{ScheduledTaskComplex,ScheduledTasksModel};





pub struct ExecuteScheduledPromptTask {
    http_client: reqwest::Client,
    interval_ms: u64,
    
    
    app_state: Arc<AppState> 
}

 
impl ExecuteScheduledPromptTask {
    pub fn new( 
        interval_ms: u64, 
        app_state:   Arc<AppState>
        ) -> Self {
        Self { 
            http_client: reqwest::Client::new(),
            interval_ms,
            
            app_state 
            }
    }

    /*

    curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [
      {
        "role": "system",
        "content": "You are a helpful assistant."
      },
      {
        "role": "user",
        "content": "Hello!"
      }
    ]
  }'

    */
    pub async fn fetch_chat_completion(&self, task_complex: &ScheduledTaskComplex ) -> Result< CompletionOutputBody ,ServiceTaskError>{
        let client = &self.http_client;
          
        let prompt_base = task_complex.plan.prompt_base.clone();
        //hit chat gpt ! 
 

        let api_url = "https://api.openai.com/v1/chat/completions";
        
            //load me from env  
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");


        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());
    
        let messages = vec![
            CompletionMessage { role: "system".to_string(), content: "You are a helpful assistant.".to_string() },
            CompletionMessage { role: "user".to_string(), content:  prompt_base  }
        ];
    
        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages
        }); 

        println!("Headers: {:?}", headers);
        println!("Request body: {:?}", body);
    
        let res = client.post(api_url)
                        .headers(headers)
                        .json(&body)
                        .send()
                        .await?;
    
        let body: serde_json::Value = res.json().await?;
        println!("{:#?}", body);
        
        
        let completion_body:CompletionOutputBody = serde_json::from_value(  body )?;

        
        Ok(  completion_body  )  
    }
}

impl ServiceInterval for ExecuteScheduledPromptTask {
    fn get_interval_ms(&self) -> u64 {
        self.interval_ms 
    }
}

#[async_trait]
impl Runnable for ExecuteScheduledPromptTask {
    

    async fn run(&mut self) {
      
        println!("Running ExecuteScheduledPromptTask with interval: {} ms", self.interval_ms);
        
        let db: &Database = &self.app_state.database; 

        let limit = 200; 

        let scheduled_tasks_result = ScheduledTasksModel::find_all_scheduled_tasks_complex(
             limit, db ).await;

        match scheduled_tasks_result {
            Ok(scheduled_tasks) => {

                for task_complex in scheduled_tasks {

                    if task_complex.scheduled_task.is_overdue_to_execute() {
                        
                        let prompt_response = self.fetch_chat_completion(
                            &task_complex
            
                        ).await;
                        
                        let scheduled_task_id = &task_complex.scheduled_task.id;
                        
                        ScheduledTasksModel::update_executed_at(  *scheduled_task_id, db ).await;
                        
                        match prompt_response {
                            Ok(completion_body_raw) => {
                                
                                  
                                  
                                let completion_output_insert = CompletionOutputsModel::insert_one(
                                    task_complex.scheduled_task.id, 
                                    completion_body_raw, 
                                    db
                                    ).await;
                                    
                                    //maybe do something if insert fails ? 
                                
                            },
                            Err( e ) => {
                                
                                println!("Open ai error {:?}", e)
                            }
                            
                        //    ScheduledTasksModel::update_executed_at( task_complex.scheduled_task, db );
                            
                            
                        }

                        
                        //NEED TO update the task db record to denote that it has been executed ! so we dont do it again 
                        
                       

                    } 
        
                }

            },
            Err(e) => println!("Find tasks error: {:?}", e)

        }

      
        
      
        
         
        
    }
}