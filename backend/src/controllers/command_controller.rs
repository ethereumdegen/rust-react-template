use std::collections::HashMap;

use crate::AppState;

use crate::ai_modules::system_command_module::{EndpointMethodType, SystemCommand};

use reqwest::Response;
use serde::{Deserialize, Serialize};
use url::Url;

use super::web_controller::WebController;

use actix_web::web::{self, Data, Json, ServiceConfig};

use crate::util::backend_server_error::BackendServerError;

use crate::ai_modules::system_command_module::CommandDomain;

pub struct CommandController {}

impl CommandController {}

impl WebController for CommandController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api/command")
                // Add your routes here, e.g.,
                .route("/confirm", web::post().to(confirm_command)),
        );
    }
}

#[derive(Deserialize, Debug)]
pub struct CommandInput {
    pub command_name: String,
    pub command_arguments: serde_json::Value,

    pub access_token: String,
    pub access_domain: String,
}

#[derive(Deserialize, Debug)]
pub struct ConfirmCommandInput {
    pub command: CommandInput,
}

#[derive(Serialize)]
pub struct ConfirmCommandResponse {
    pub command_name: String,
    pub endpoint_data: Option<serde_json::Value>,
    pub success: bool,
}

async fn confirm_command(
    params: Json<ConfirmCommandInput>,
    _app_state: Data<AppState>,
) -> actix_web::Result<Json<ConfirmCommandResponse>> {
    println!("\n confirmed command {:?} \n", params);

    let command_input = &params.command;

    let app_endpoint = CommandDomain::try_from((&command_input.access_domain).clone())
        .map_err(|_e| BackendServerError::InputParsingError)?;

    let domain_url_root = app_endpoint.get_domain_url();

    let mut command_args_val: serde_json::Value = (&command_input.command_arguments).clone();

    if let Some(obj) = command_args_val.as_object_mut() {
        // Add a new key-value pair
        obj.insert(
            "auth_token".to_string(),
            serde_json::Value::String(command_input.access_token.clone()),
        );
    }

    let command_name = &command_input.command_name.clone();

    let system_command = SystemCommand::try_from_function_name(&command_input.command_name)
        .map_err(|_e| BackendServerError::InputParsingError)?;

    let (endpoint_method_type, api_endpoint_suffix) =
        system_command.get_api_endpoint_method_type_and_suffix();

    let complete_endpoint_url = domain_url_root
        .join(&api_endpoint_suffix)
        .map_err(|_e| BackendServerError::InputParsingError)?;

    //could put this in its own fn !
    let endpoint_response: Response = perform_request(
        &endpoint_method_type,
        &complete_endpoint_url,
        &command_args_val,
    )
    .await?;

    let response_data_text = endpoint_response
        .text()
        .await
        .map_err(|_e| BackendServerError::UnknownError)?;
    println!("response_data !!  {:?}", response_data_text);

    let endpoint_data: Option<serde_json::Value> = serde_json::from_str(&response_data_text)
        .map_err(|_e| BackendServerError::UnknownError)
        .ok();

    println!("was able to parse response_data {:?}", endpoint_data);

    let res = ConfirmCommandResponse {
        command_name: command_name.to_string(),
        endpoint_data: endpoint_data,
        success: true,
    };

    Ok(Json(res))
}

async fn perform_request(
    endpoint_method_type: &EndpointMethodType,
    complete_endpoint_url: &Url,
    command_args_val: &serde_json::Value,
) -> Result<Response, BackendServerError> {
    let client = reqwest::Client::new();

    let response = match endpoint_method_type {
        EndpointMethodType::POST => client
            .post(complete_endpoint_url.clone())
            .json(&command_args_val)
            .send()
            .await
            .map_err(|_e| BackendServerError::UnknownError)?,

        _ => {
            //do a GET

            if let Some(obj) = command_args_val.as_object() {
                // Initialize a HashMap to hold query parameters
                let mut query_params_str: HashMap<String, String> = HashMap::new();

                // Populate HashMap
                for (k, v) in obj.iter() {
                    let val_str = match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => continue, // Ignore other types
                    };
                    query_params_str.insert(k.clone(), val_str);
                }

                // Make the HTTP GET request with query parameters

                let response = client
                    .get(complete_endpoint_url.clone())
                    .query(&query_params_str)
                    .send()
                    .await
                    .map_err(|_e| BackendServerError::UnknownError)?;

                response
            } else {
                println!("WARN : COULD NOT DO GET REQ ");
                return Err(BackendServerError::UnknownError.into());
            }
        }
    };

    Ok(response)
}
