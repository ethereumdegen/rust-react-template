
 /*
pub mod price_oracle_task;

pub mod read_paid_invoice_events_task;


pub mod invoice_status_update_task;
*/
pub mod execute_scheduled_prompt_task;
 
pub mod deliver_completion_outputs_task;


use ethers::{  prelude::{  ContractError}, providers::{  Provider,Http}};
 
 
#[derive(thiserror::Error,Debug)]
pub enum ServiceTaskError{
 
    #[error("Record does not exist")]
    RecordDoesNotExist,
    
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    
    #[error("Ethers ABI error: {0}")]
    AbiError(#[from] ethers::abi::AbiError),
    
   #[error("Ethers Contract error: {0}")]
    EthersContractError(#[from] ContractError<Provider<Http>>),
    
    
    #[error("Serde Json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}