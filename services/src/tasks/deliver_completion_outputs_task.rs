/*

This task will execute a scheduled prompt.

It will hit Chat Gpt with a combined query of :

1. the prompt in the associated 'plan' that was scheduled 
2. user-specific context data for personalization


The record will then be saved in postgres and somehow given to the user (email?app?text?)



https://platform.openai.com/docs/guides/gpt/chat-completions-api

*/


use async_trait::async_trait;

use crate::{Runnable, service_task::ServiceInterval, db::postgres::models::{completion_output_model::{CompletionOutputBody, CompletionOutputsModel, CompletionOutputComplex}, user_auth_tokens_model::UserAuthTokensModel}};
use serde::{Serialize,Deserialize};
use serde_json::json;




use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use std::{env, collections::HashMap};

use degen_sql::db::postgres::postgres_db::Database;
use crate::tasks::ServiceTaskError;
use crate::AppState;

use std::sync::Arc;


use crate::db::postgres::models::scheduled_task_model::{ScheduledTaskComplex,ScheduledTasksModel};

 
 
 #[derive(Serialize, Deserialize,Debug)]
pub struct TweetResponse {
    pub id: String,
   
    // add any other fields that Twitter's OAuth2 response contains
}



pub struct DeliverCompletionOutputsTask {
    http_client: reqwest::Client,
    interval_ms: u64,
    
    
    app_state: Arc<AppState> 
}

 
impl DeliverCompletionOutputsTask {
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

   
    pub async fn deliver_completion_output(
        &self, 
        completion_output_complex: &CompletionOutputComplex ,
        db: &Database
        
          ) 
    -> Result<  () ,ServiceTaskError>{
        
        
        //need to find the delivery type like twitter
        
        let delivery_service_name = completion_output_complex.delivery_method.name.clone(); 
        
        //need to find the users auth bearer token 
        
        
        let user_id = completion_output_complex.user.id;
        
        let auth_tokens = UserAuthTokensModel::find_valid_auth_token_for_user(
             user_id, 
             delivery_service_name,
             db 
        ).await.map_err(|_e| ServiceTaskError::RecordDoesNotExist )?; 
        
        
        
        let auth_token_record = auth_tokens.first().ok_or( ServiceTaskError::RecordDoesNotExist  ) ?;
        
        let auth_token = auth_token_record.auth_token.clone();
        
        println!("about to send tweet with auth token {}", auth_token);
         
         
         let open_ai_completion = &completion_output_complex.completion_output.body;
         
         
         let first_choice = open_ai_completion.choices.first();
         
         if let Some(first_choice) = first_choice {
             
              
            let tweet_text = &first_choice.message.content;
            
            let tweet_response = self.send_tweet(tweet_text,&auth_token).await ?;
            
         
         
         }
          
        
        
        Ok(())
    }
    
    
    
    
    pub async fn send_tweet(  
        &self,
        tweet_text: &String, 
        auth_token: &String 
    ) ->Result<  () , ServiceTaskError > {
        
        
         let action_url:String =   "https://api.twitter.com/2/tweets" .into() ;

        
         
            //  https://developer.twitter.com/en/docs/authentication/oauth-2-0/authorization-code
        
        
            let client = &self.http_client; // reqwest::Client::new();
            let mut payload = HashMap::new(); 
            payload.insert("text", tweet_text);
            
        
            let auth_payload = Box::leak(format!("Bearer {}", auth_token).into_boxed_str());
        
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
            headers.insert(AUTHORIZATION, HeaderValue::from_str( auth_payload ).unwrap());
        
        
            let res = client
                .post(action_url.to_string())
                .headers(headers)
                .json(&payload)
        
                .send()
                .await.expect("REQUEST FAILED");
        
         
            println!("got res {:?}", &res );
            
             println!("got error {:?}", &res.error_for_status());
             
        
           // let token_info: TweetResponse = res.json().await ?;
           // println!("TweetResponse: {:?}", token_info);
             
            
            
            Ok( ())
    
  }
    
    
    
}

impl ServiceInterval for DeliverCompletionOutputsTask {
    fn get_interval_ms(&self) -> u64 {
        self.interval_ms 
    }
}

#[async_trait]
impl Runnable for DeliverCompletionOutputsTask {
    

    async fn run(&mut self) {
      
        println!("Running DeliverCompletionOutputsTask with interval: {} ms", self.interval_ms);
        
        let db: &Database = &self.app_state.database; 

        let limit = 200; 
        
        let undelivered_completion_outputs = CompletionOutputsModel::find_all_completion_outputs_complex(
            false, limit, db).await;
        
        
        if let Ok(outputs) =  undelivered_completion_outputs {
            
            
            for output in outputs {
                
                //deliver such as via twitter 
                
                let delivery_result = self.deliver_completion_output(&output, db).await;
                
                if let Ok(delivered) = delivery_result {
                    
                    let _update_result = CompletionOutputsModel::update_delivered_at(  output.completion_output.id, db ).await;
                    
                    
                    
                    
                }
                
            }           
            
            
        }

       
      
        
      
        
         
        
    }
}

