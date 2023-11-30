use std::env;

use oauth2::http::{HeaderMap, HeaderValue};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::json;

use crate::types::openai_completion::{
    OpenAiCallableFunction, OpenAiCompletion, OpenAiErrorResponse,
};

use super::backend_server_error::BackendServerError;

pub type CompletionOutputBody = OpenAiCompletion;

#[derive(Serialize)]
pub struct CompletionMessage {
    role: String,
    content: String,
}

pub struct OpenAiClient {
    api_url: String,

    openai_api_key: String,
    system_message: String,

    model: String,

    functions: Vec<OpenAiCallableFunction>,
}

impl OpenAiClient {
    pub fn new(system_message: Option<String>, functions: Vec<OpenAiCallableFunction>) -> Self {
        Self {
            api_url: "https://api.openai.com/v1/chat/completions".into(),
            openai_api_key: env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found"),
            system_message: system_message.unwrap_or("You are a helpful assistant.".into()),
            model: "gpt-3.5-turbo".into(),
            functions,
        }
    }

    pub async fn fetch_chat_completion(
        &self,
        prompt_base: String,
        enable_function_calling: bool,
    ) -> Result<CompletionOutputBody, BackendServerError> {
        let client = reqwest::Client::new();

        let api_url = self.api_url.clone();

        //load me from env
        let api_key = self.openai_api_key.clone();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        let messages = vec![
            CompletionMessage {
                role: "system".to_string(),
                content: self.system_message.clone(),
            },
            CompletionMessage {
                role: "user".to_string(),
                content: prompt_base,
            },
        ];

        let functions = &self.functions;

        let function_call_mode_string = match enable_function_calling {
            true => "auto",
            false => "none",
        };

        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages,
            "functions": functions,
             "function_call": function_call_mode_string.to_string() //may call a fn, might not
        });

        println!("Headers: {:?}", headers);
        println!("Request body: {:?}", body);

        let res = client
            .post(api_url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|_e| BackendServerError::UnknownError)?;

        println!(" res   {:#?}", res);

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|_e| BackendServerError::InputParsingError)?;

        println!(" res body {:#?}", body);

        let completion_body = serde_json::from_value::<CompletionOutputBody>(body.clone());
        //  .map_err(|_e| BackendServerError::InputParsingError)?;

        if let Ok(completion_message) = completion_body {
            return Ok(completion_message);
        } else {
            let response_error = serde_json::from_value::<OpenAiErrorResponse>(body.clone());

            if let Ok(res_err) = response_error {
                println!("Err: {:?}", res_err);
            }

            return Err(BackendServerError::UnknownError.into());
        }

        //Ok(  completion_body  )
    }
}

/*

pub async fn fetch_chat_completion(
        user_input: String,
        functions: Vec<OpenAiCallableFunction>

          ) -> Result<
    CompletionOutputBody ,
    BackendServerError

    >{
        let client = reqwest::Client::new();

       // let prompt_base = task_complex.plan.prompt_base.clone();
        //hit chat gpt !


        let api_url = "https://api.openai.com/v1/chat/completions";

            //load me from env
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found");


        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

        let messages:Vec<CompletionMessage> = vec![
            CompletionMessage { role: "system".to_string(), content: "You are a helpful assistant.".to_string() },
            CompletionMessage { role: "user".to_string(), content:  user_input  }
        ];

       // let functions: Vec<CompletionFunction> = get_callable_functions

        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages,
            "functions" : functions,
            "function_call": "auto".to_string() //may call a fn, might not
        });

        println!("Headers: {:?}", headers);
        println!("Request body: {:?}", body);

        let res = client.post(api_url)
                        .headers(headers)
                        .json(&body)
                        .send()
                        .await.map_err(|_e| BackendServerError::UnknownError)?;

        let body: serde_json::Value = res.json().await
        .map_err(|_e|BackendServerError::InputParsingError)?;
        println!("{:#?}", body);


        let completion_body:CompletionOutputBody = serde_json::from_value(  body )
        .map_err(|_e| BackendServerError::InputParsingError)?;


        Ok(  completion_body  )
    }

    */
