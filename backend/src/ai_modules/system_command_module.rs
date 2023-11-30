use std::collections::HashMap;

use crate::types::openai_completion::{GptFunctionCall, OpenAiCallableFunction};

use super::AiSystemModule;

use serde::{self, Deserialize, Serialize};

use serde_json::Value;

use reqwest::Url;

// https://platform.openai.com/docs/guides/gpt/function-calling

/*

i would like to enter an order .   i want to add SKU  234 , quantity of 1 , for the company Pepsi .



Please show me all of the sales orders for the company microsoft from Dec 17, 2020 to Dec 20, 2020


*/

/*




*/

pub const sales_order_module_schema_string: &str = include_str!("sales_order_module_schema.json");

pub const indiefuture_module_schema_string: &str = include_str!("indiefuture_module_schema.json");

#[derive(Debug, thiserror::Error)]
pub enum SystemCommandModuleError {
    #[error("unknown function name")]
    UnknownFunctionName,

    #[error("JSON parsing error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub enum EndpointMethodType {
    GET,
    POST,
}

pub enum CommandDomain {
    Indiefuture,
}

impl CommandDomain {
    pub fn get_domain_url(&self) -> Url {
        match self {
            Self::Indiefuture => Url::parse("https://api.indiefuture.com").unwrap(),
            //Self::Indiefuture => Url::parse("http://localhost:8000").unwrap(),
        }
    }
}

impl TryFrom<String> for CommandDomain {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "indiefuture" => Ok(CommandDomain::Indiefuture),
            _ => Err("Invalid string for CommandEndpoint".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub enum SystemCommand {
    // AddSalesOrderEntry (OrderEntryResponseComplex),
    // FindSalesOrders (FindOrdersResponseComplex),
    FindProducts,
    FindProduct,
    FindClassifications,
    UpdateProduct,
}

impl SystemCommand {
    pub fn get_api_endpoint_method_type_and_suffix(&self) -> (EndpointMethodType, String) {
        match self {
            Self::FindProducts => (
                EndpointMethodType::POST,
                "/api/product/find_all_by_owner".into(),
            ),
            Self::FindProduct => (EndpointMethodType::GET, "/api/product".into()),

            Self::FindClassifications => (
                EndpointMethodType::GET,
                "/api/product/classifications".into(),
            ),
            Self::UpdateProduct => (EndpointMethodType::POST, "/api/product/update".into()),
        }
    }

    pub fn try_from_gpt_function_call(
        function_call: &GptFunctionCall,
    ) -> Result<SystemCommand, SystemCommandModuleError> {
        let fn_name = &function_call.name;

        let args_str = &function_call.arguments;
        let _args_json: Value = serde_json::from_str(args_str)?;

        let system_command = SystemCommand::try_from_function_name(fn_name /*, args_json */)?;

        return Ok(system_command);
    }

    pub fn try_from_function_name(
        fn_name: &str, /*, args_json: Value*/
    ) -> Result<Self, SystemCommandModuleError> {
        println!("fn_name {:?}", fn_name);

        match fn_name {
            "FindProducts" => Ok(Self::FindProducts),

            "FindProduct" => Ok(Self::FindProduct),

            "FindClassifications" => Ok(Self::FindClassifications),

            "UpdateProduct" => Ok(Self::UpdateProduct),

            _ => Err(SystemCommandModuleError::UnknownFunctionName),
        }
    }
} //impl

pub struct GenericSystemModule {
    function_definitions: HashMap<String, OpenAiCallableFunction>,

    system_message: Option<String>,
}

impl GenericSystemModule {
    pub fn new() -> Self {
        Self {
            function_definitions: HashMap::new(),
            system_message: None,
        }
    }

    pub fn set_system_message(mut self, msg: String) -> Self {
        self.system_message = Some(msg);

        self
    }

    //sets from json
    pub fn set_function_definitions(
        mut self,
        json_str: &str,
    ) -> Result<Self, SystemCommandModuleError> {
        let functions_array: Vec<OpenAiCallableFunction> = serde_json::from_str(&json_str)?;

        let mut function_definitions = HashMap::new();

        for func in functions_array {
            function_definitions.insert(func.name.clone(), func);
        }

        self.function_definitions = function_definitions;

        Ok(self)
    }
}

impl AiSystemModule for GenericSystemModule {
    fn get_system_message(&self) -> Option<String> {
        self.system_message.clone()
    }

    fn get_functions(&self) -> Vec<OpenAiCallableFunction> {
        let mut functions_array = Vec::new();

        for value in self.function_definitions.values() {
            functions_array.push(value.clone());
        }

        functions_array
    }
}

// TYPES

pub struct FunctionDefinition {
    // function_name: String,
    callable_function: OpenAiCallableFunction,
    // could be OrderEntryResponseComplex or FindOrdersResponseComplex

    //gpt_response_definition:  SystemCommand
}

/*
#[derive(Debug, Serialize,Deserialize)]
pub struct FindOrdersResponseComplex{
    company_name: Option<String>,
    start_datetime: AlphaNumericString ,
    end_datetime: AlphaNumericString
}



#[derive(Debug, Serialize,Deserialize)]
pub struct OrderEntryResponseComplex{
    company_name: String,
    order_entries: Vec<OrderEntry>
}

#[derive(Debug, Serialize ,Deserialize)]
pub struct OrderEntry {

    SKU: AlphaNumericString,
   // company_name: String,
    quantity: AlphaNumericString

}

*/

// --- PARAMETERS ---

/*
impl OrderEntriesParameter {




    pub fn new() -> Self {

        let mut properties:HashMap<String,Value> = HashMap::new();

       properties.insert("SKU".into(), SimpleFunctionParameter {
            type_field: "string".into(),
            description: Some("The SKU identifier".into()),
            enum_field: vec![],
            required: vec![]
        }.to_serde_value());

     /*   properties.insert("customer_name".into(), SimpleFunctionParameter {
            type_field: "string".into(),
            description: Some("The name of the customer".into()),
            enum_field: vec![],
            required: vec![]
        }.to_serde_value()); */

        properties.insert("quantity".into(), SimpleFunctionParameter {
            type_field: "integer".into(),
            description: Some("The quantity of the item".into()),
            enum_field: vec![],
            required: vec![]
        }.to_serde_value());


        let required = vec!["SKU".into(),  "quantity".into()];


        Self{
            type_field: "array".into(),
            description: Some( "An array of order entries".into() ),
            items:  OpenAiCallableParameters{
                type_field:"object".into(),
                properties,
                required

            }
        }
    }

    pub fn to_serde_value(&self) -> serde_json::Value {
        serde_json::to_value( self ).unwrap()
    }

}
*/
