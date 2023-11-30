use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GptFunctionCall {
    pub name: String,
    pub arguments: String, // serde_json::Value  //this a stringified array i think         // Vec< Box<serde_json::Value> >
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: Option<String>,
    pub role: Option<String>,
    pub function_call: Option<GptFunctionCall>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub finish_reason: String,
    pub index: u32,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiCompletion {
    pub choices: Vec<Choice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiError {
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiErrorResponse {
    pub error: OpenAiError,
}

// ------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiCallableFunction {
    pub name: String,
    pub description: String,
    pub parameters: OpenAiCallableParameters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiCallableParameters {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: HashMap<String, serde_json::Value>, //Box<serde_json::Value> >,  // value is actually a property descriptor
    pub required: Vec<String>,                          //these are the required properties
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleFunctionParameter {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_field: Vec<String>,
    pub required: Vec<String>,
}

impl SimpleFunctionParameter {
    pub fn to_serde_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

//this is just an example !
/*
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDescriptor {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: Option<String>,
}*/

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub location: Location,
    pub unit: Option<Unit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}*/

/*
#[derive(Debug, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "celsius")]
    Celsius,

    #[serde(rename = "fahrenheit")]
    Fahrenheit,
}*/
