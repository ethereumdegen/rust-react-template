


use ethers::{types::Address, utils::to_checksum};

use chrono::{DateTime, NaiveDateTime, Utc};

use degen_sql::db::postgres::postgres_db::Database; 

 
use degen_sql::db::postgres::models::model::PostgresModelError;

use tokio_postgres::Row;
use cron::Schedule;
 
use std::str::FromStr;
use std::time::Duration;

use crate::db::postgres::domains::eth_address::DomainEthAddress;

 
 


pub struct User {
    pub id: i32,
    pub public_address: Address
}

impl User {

    pub fn from_joining_row(row: &Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("user_id");
      
        let public_address =  row.try_get::<_, DomainEthAddress>("user_public_address")?.0;


        Ok(Self {
            id,
            public_address
        })
    }

    pub fn get_select_query() -> String {
        "SELECT 
        user.id AS user_id,
        user.name AS user_name
        ".into()
    }
}

pub struct UsersModel {}

impl UsersModel { 

 

}


 