use chrono::{DateTime, Utc};
 
use ethers::{types::{Address, U256, H256}};
use tokio_postgres::Row;

use degen_sql::db::postgres::postgres_db::Database;
use degen_sql::db::postgres::models::model::PostgresModelError;

use crate::{
    db::postgres::{domains::{eth_address::DomainEthAddress, uint256::DomainUint256, bytes32::DomainBytes32}
        
        
    },
    types::{chronodatetime::ChronoDateTime},
};

 
use std::str::FromStr;

use serde::Serialize;
 

/*
  id SERIAL PRIMARY KEY,


  contract_address eth_address NOT NULL,

  chain_id uint256 NOT NULL,



  token eth_address NOT NULL,

  total_amount uint256 NOT NULL,

  nonce uint256 NOT NULL,

  expires_at TIMESTAMPTZ NOT NULL,


  invoice_uuid bytes32 UNIQUE NOT NULL ,


  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
*/
#[derive(Serialize)]
pub struct Invoice {
    pub id: i32,

    pub parent_user_id: Option<i32>,

    pub contract_address: Address,

    pub token: Address,
    pub total_amount: U256,

    pub nonce: U256,
    pub chain_id: U256,
    pub expires_at: ChronoDateTime,

    pub metadata_hash: H256,
    pub invoice_uuid: H256,

    pub created_at: ChronoDateTime,
}

impl Invoice {
    
    
    pub fn from_row(row: &Row) -> Result<Self, PostgresModelError> {
        let invoice_uuid_string = row.get::<_, String>("invoice_uuid");
        let metadata_hash_string = row.get::<_, String>("metadata_hash");

        let invoice = Self {
            id: row.try_get("id")?,
            parent_user_id: row.try_get("parent_user_id").ok(),
            contract_address: row.try_get::<_, DomainEthAddress>("contract_address")?.0,
            token: row.try_get::<_, DomainEthAddress>("token")?.0,
            total_amount: row.try_get::<_, DomainUint256>("total_amount")?.0,
            nonce: row.try_get::<_, DomainUint256>("nonce")?.0,
            chain_id: row.try_get::<_, DomainUint256>("chain_id")?.0,
            expires_at: ChronoDateTime::new(row.try_get("expires_at")?),
            metadata_hash: H256::from_str(&metadata_hash_string)
                .map_err(|_e| PostgresModelError::RowParseError)?,
            invoice_uuid: H256::from_str(&invoice_uuid_string)
                .map_err(|_e| PostgresModelError::RowParseError)?,
            created_at: ChronoDateTime::new(row.try_get("created_at")?),
        };

        Ok(invoice)
    }
}

pub struct InvoicesModel {}

impl InvoicesModel {
    /*


    hash VARCHAR(255) NOT NULL,
    parent_public_address VARCHAR(255) NOT NULL,

     */

    pub async fn find_or_create_invoice(
        parent_user_id: i32,

        contract_address: Address,
        chain_id: U256,

        token: Address,
       
        total_amount: U256,
        nonce: U256,

        expires_at: DateTime<Utc>,

        metadata_hash: H256,
        invoice_uuid: H256,

        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
        let insert_result = psql_db
            .query_one(
                "INSERT INTO invoices 
            ( 
              parent_user_id, 
              contract_address, 
              chain_id,
           
              token,
              total_amount,
              nonce,
              expires_at,
              metadata_hash,
              invoice_uuid 
             )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9 )
            ON CONFLICT(invoice_uuid) DO NOTHING
            RETURNING id;
            ",
                &[
                    &parent_user_id,
                    &DomainEthAddress(contract_address),
                    &DomainUint256(chain_id),
                    &DomainEthAddress(token),
                    &DomainUint256(total_amount),
                    &DomainUint256(nonce),
                    &expires_at,
                    &DomainBytes32(metadata_hash.0),
                    &DomainBytes32(invoice_uuid.0),
                ],
            )
            .await;

        match insert_result {
            Ok(row) => Ok(row.get("id")), // Successfully inserted new product and got its ID.
            Err(e) => {
                eprintln!("{}", e);
                // Conflict occurred and no new product was inserted. Fetch the existing product's ID.
                let existing_row = psql_db
                    .query_one(
                        "SELECT id FROM invoices WHERE invoice_uuid = $1",
                        &[&DomainBytes32(invoice_uuid.0)],
                    )
                    .await?;

                Ok(existing_row.get("id"))
            }
        }
    }
    
     pub async fn update_invoice_statuses_to_paid(
        invoice_uuids: Vec<String>, 
        psql_db: &Database,
    ) -> Result< (), PostgresModelError> {
        let update_result = psql_db
            .query(
                "UPDATE invoices 
                SET payment_status = 'paid'
                WHERE invoice_uuid = ANY($1)",
                &[
                    &invoice_uuids
                ],
            )
            .await;

         Ok(())
    }
    
}
