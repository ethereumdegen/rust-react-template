


use ethers::{types::Address, utils::to_checksum};

use chrono::{DateTime, NaiveDateTime, Utc};

use degen_sql::db::postgres::postgres_db::Database; 

 
use degen_sql::db::postgres::models::model::PostgresModelError;

use tokio_postgres::Row;
use cron::Schedule;
 
use std::str::FromStr;
use std::time::Duration;


 
 


pub struct DeliveryMethod {
    pub id: i32,
    pub name: String
}

impl DeliveryMethod {

    pub fn from_joining_row(row: &Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("delivery_method_id");
        let name: String = row.get("delivery_method_name");

        Ok(Self {
            id,
            name
        })
    }

    pub fn get_select_query() -> String {
        "SELECT 
        delivery_method.id AS delivery_method_id,
        delivery_method.name AS delivery_method_name
        ".into()
    }
}

pub struct DeliveryMethodsModel {}

impl DeliveryMethodsModel { 

 

}


 