use crate::ai_modules::system_command_module::{
    indiefuture_module_schema_string, GenericSystemModule, SystemCommand,
};
use crate::ai_modules::AiSystemModule;
use crate::AppState;

use serde::{Deserialize, Serialize};

use super::web_controller::WebController;

use actix_web::web::{self, Data, Json, ServiceConfig};

use crate::util::openai::OpenAiClient;

pub struct ChatController {}

impl ChatController {}

//POST http://localhost:8000/api/order/create

impl WebController for ChatController {
    fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            web::scope("/api/chat")
                // Add your routes here, e.g.,
                .route("", web::get().to(get_chat_input)),
        );
    }
}

//for twitter !!

#[derive(Deserialize)]
pub struct GetChatInput {
    pub body: String,
    //auth token ?
    //pub code: String
}

#[derive(Serialize)]
pub struct GetChatResponse {
    pub command: Option<SystemCommandResponse>, //could be an enum ?? w message in some of them.
    pub message: Option<String>,
    //other stuff?
}

#[derive(Serialize)]
pub struct SystemCommandResponse {
    pub command_name: Option<SystemCommand>,
    pub arguments: Option<serde_json::Value>,
}

async fn get_chat_input(
    chat_input: web::Query<GetChatInput>,
    _app_state: Data<AppState>,
) -> actix_web::Result<Json<GetChatResponse>> {
    // let db = &app_state.database;

    println!("chat input body {}", chat_input.body);

    //send the chat body to chat gpt !

    let user_input = chat_input.body.clone();

    let sales_order_module = GenericSystemModule::new()
    .set_system_message(
        "You are an assistant for an account manager managing their ecommerce website. 
          You are able to help them retrieve information about their products, orders and sales by translating their raw text into function calls.
        Feel free to ask for more information if required to call the functions.
        
        Please be very aware that while many input parameters are optional (you are allowed to use null) you should be very wise and intelligent with your use of this.
        Err on the of NOT using NULL and use a value, such as 0, when appropriate especially when performing updates.  
        
        ".into() 
             
             )
    .set_function_definitions( indiefuture_module_schema_string  ).unwrap();

    let functions = sales_order_module.get_functions();

    let system_message = sales_order_module.get_system_message();

    // -----

    let open_ai_client = OpenAiClient::new(system_message, functions);

    let completion_response = open_ai_client.fetch_chat_completion(user_input, true).await;

    println!("completion response {:?}", completion_response);
    //we should make sure the state matches what we sent to prevent cross script forging
    //we should store the user auth token in the db ! we can use it now to tweet for them .

    /*let oauth_token = oauth_callback_input.code.clone();
    println!("oauth recvd: {} {} ", &oauth_callback_input.code,
    &oauth_callback_input.state);*/

    let gpt_function_call = completion_response
        .as_ref()
        .ok()
        .map(|completion| {
            completion
                .choices
                .first()
                .map(|choice| choice.message.function_call.clone())
        })
        .flatten()
        .flatten();

    let gpt_message_content = completion_response
        .as_ref()
        .ok()
        .map(|completion| {
            completion
                .choices
                .first()
                .map(|choice| choice.message.content.clone())
        })
        .flatten()
        .flatten();

    println!("gpt fn call {:?}", gpt_function_call);

    let system_command = gpt_function_call
        .as_ref()
        .map(|cmd| SystemCommand::try_from_function_name(&cmd.name).ok())
        .flatten();

    let system_command_response = gpt_function_call.as_ref().map(|c| SystemCommandResponse {
        command_name: system_command,
        arguments: serde_json::from_str(&c.arguments).ok(),
    });

    //.map( |call|   SalesOrderModule::parse_gpt_function_call( call )     )  ;

    let res = GetChatResponse {
        command: system_command_response,
        message: gpt_message_content,
    };

    Ok(Json(res))
}

/*

  let connect_twitter_oauth_properties = HashMap::new();
   OpenAiCallableFunction{
           name:"connect_twitter_oauth".into(),
           description:"initializes the user by prompting them to log in to twitter oauth".into(),
           parameters: OpenAiCallableParameters{
               type_field:"object".into(),
               properties: connect_twitter_oauth_properties,
               required: vec![]

           }

       }


*/
