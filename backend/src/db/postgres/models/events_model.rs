use ethers::{types::Address, utils::to_checksum};

use chrono::{DateTime, NaiveDateTime, Utc};

use degen_sql::db::postgres::postgres_db::Database; 

use crate::types::contract_event::ContractEvent;

use degen_sql::db::postgres::models::model::PostgresModelError;



pub struct EventsModel {}

impl EventsModel {
    pub async fn insert_one(
        event: &ContractEvent,

        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
        let contract_address = to_checksum(&event.address, None).to_string();

        let name = &event.name;

        let signature = serde_json::to_string(&event.signature).unwrap();

        let args = serde_json::to_string(&event.args).unwrap();

        let data = serde_json::to_string(&event.data).unwrap();

        let transaction_hash = serde_json::to_string(&event.transaction_hash).unwrap();

        let block_hash = serde_json::to_string(&event.block_hash).unwrap();

        let block_number: &String = &event.block_number.unwrap().low_u64().to_string();

        let log_index: i64 = event.log_index.unwrap().low_u64() as i64;
        
        let chain_id = event.chain_id.to_string();

        let transaction_index: i64 = event.transaction_index.unwrap().low_u64() as i64;

        let insert_result = psql_db
            .query_one(
                "
            INSERT INTO events 
            (
            contract_address,
            name,
            signature,
            args,
            data,
            transaction_hash,
            block_number,
            chain_id,
            block_hash,
            log_index,
            transaction_index            
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id;
            ",
                &[
                    &contract_address,
                    &name,
                    &signature,
                    &args,
                    &data,
                    &transaction_hash,
                    &block_number,
                    &chain_id,
                    &block_hash,
                    &log_index,
                    &transaction_index,
                ],
            )
            .await;

        match insert_result {
            Ok(row) => Ok(row.get(0)), // Successfully inserted new row and got its ID.
            Err(e) => {
                eprintln!("Database error: {:?}", e);

                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn find_most_recent_event(
        contract_address: Address,
        psql_db: &Database,
    ) -> Result<ContractEvent, PostgresModelError> {
        let parsed_contract_address = to_checksum(&contract_address, None).to_string();

        let row = psql_db
            .query_one(
                "
        SELECT 
            contract_address,
            name,
            signature,
            args,
            data,
            transaction_hash,
            block_number,
            block_hash,
            chain_id,
            log_index,
            transaction_index,
            created_at
        FROM events
        WHERE (contract_address) = ($1)
        ORDER BY created_at DESC
        LIMIT 1;
        ",
                &[&parsed_contract_address],
            )
            .await;

        match row {
            Ok(row) => {
                
                 let event = ContractEvent::from_row(row)?;
                 
                 
                Ok(event)
            }
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }

    pub async fn find_events(
        contract_address: Address,
        event_name: String,
        start_at_created_at: Option<DateTime<Utc>>,
        limit: u32,
        psql_db: &Database,
    ) -> Result<Vec<ContractEvent>, PostgresModelError> {
        let default_start_at = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDateTime::from_timestamp_millis(0).unwrap(),
            Utc,
        );
        let start_at = start_at_created_at.unwrap_or(default_start_at);

        let parsed_contract_address = to_checksum(&contract_address, None).to_string();

        let rows = psql_db
            .query(
                "
        SELECT 
            contract_address,
            name,
            signature,
            args,
            data,
            transaction_hash,
            block_number,
            block_hash,
            chain_id,
            log_index,
            transaction_index,
            created_at
        FROM events
        WHERE (contract_address = $1 AND name = $2  AND created_at > $3)
        ORDER BY created_at ASC
        LIMIT $4;
        ",
                &[
                    &parsed_contract_address,
                    &event_name,
                    &start_at,
                    &(limit as i64),
                ],
            )
            .await;

        match rows {
            Ok(rows) => {
                let mut events = Vec::new();

                for row in rows {
                    
                    let event = ContractEvent::from_row(row)?;
                    
                    

                    events.push(event)
                }

                Ok(events)
            }
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                Err(PostgresModelError::Postgres(e))
            }
        }
    }
}
