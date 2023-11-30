use std::str::FromStr;

use serde::{Serialize };

use tokio_postgres::{Row};

use crate::db::postgres::domains::eth_address::DomainEthAddress;

use degen_sql::db::postgres::postgres_db::Database;
use degen_sql::db::postgres::models::model::PostgresModelError;

use super::JoinQueryRow;
use super::access_tokens_model::AccessTokensModel;

use ethers::types::Address;
use ethers::utils::to_checksum;

#[derive(Serialize,Debug)]
pub struct User {
    pub id: i32,
    pub name : Option<String> ,
    pub public_address: Address,
    pub roles: Vec<String> 
}

impl User {
    
    pub fn has_role( &self, role_name: &String ) -> bool {
        
        self.roles.contains(role_name)
    }
    
    fn from_row(row: &Row) -> Result<Self, PostgresModelError> {
        let address = row.get::<_, String>("public_address");

        Ok(Self {
            id: row.get::<_, i32>("id"),
            name: row.try_get("name").ok(),
            roles: convert_roles_string_to_array ( row.try_get::<_,String>("roles").ok() ),
            public_address: Address::from_str(&address)
                .map_err(|_e| PostgresModelError::RowParseError)?,
        })
    }
    
    /*
    Remember that 'user' is a reserved word in postgres..
    */
    pub fn from_joining_row(row: &Row, prefix: String) -> Result<Self, PostgresModelError> {
        let address = row.get::<_, String>(format!("{}_public_address", prefix).as_str() );

        Ok(Self {
            id: row.get::<_, i32>(format!("{}_id", prefix).as_str()),
            name: row.try_get(format!("{}_name", prefix).as_str()).ok(),
            roles:  convert_roles_string_to_array ( row.try_get::<_,String>(format!("{}_roles", prefix).as_str()).ok() ),
            public_address: Address::from_str(&address)
                .map_err(|_e| PostgresModelError::RowParseError)?,
        })
    }
    
     pub fn get_query_rows(prefix: String) -> Vec<JoinQueryRow> { 
         vec![
        JoinQueryRow::new(&format!("{}.id", prefix), &format!("{}_id", prefix)),
        JoinQueryRow::new(&format!("{}.name", prefix), &format!("{}_name", prefix)),
        JoinQueryRow::new(&format!("{}.roles", prefix), &format!("{}_roles", prefix)),
        JoinQueryRow::new(&format!("{}.public_address", prefix), &format!("{}_public_address", prefix))
        ]
    }
}

pub struct UsersModel {}

impl UsersModel {
    pub async fn find_or_create_user(
        public_address: Address,
        name: Option<String>, 
        psql_db: &Database,
    ) -> Result<User, PostgresModelError> {
        let formatted_address = to_checksum(&public_address, None).to_string();

        
        let name = name.unwrap_or_else(|| formatted_address[0..8].to_owned());
    
        println!("{:?}" , formatted_address);
        
        let domain_address = DomainEthAddress(public_address);
        
        
        
        println!("{}" , "
            INSERT INTO users 
            ( public_address, name  ) 
            VALUES ( $1 , $2 )
             ON CONFLICT(public_address) DO NOTHING
            RETURNING id, name, public_address, roles;
            ");

        let insert_result = psql_db
            .query_one(
                "
            INSERT INTO users 
            ( public_address, name  ) 
            VALUES ( $1 , $2 )
             ON CONFLICT(public_address) DO NOTHING
            RETURNING id, name, public_address, roles;
            ",
                &[& domain_address , &name],
            )
            .await;

        match insert_result {
            Ok(row) => User::from_row(&row), // Successfully inserted new product and got its ID.
            Err(_) => {
                
                    println!("user alrdy exists ");
                
                // Conflict occurred and no new product was inserted. Fetch the existing product's ID.
                let existing_row = psql_db
                    .query_one(
                        "SELECT id, name, public_address, roles  
                        FROM users WHERE public_address = $1",
                        &[&  formatted_address],
                    )
                    .await?;

                User::from_row(&existing_row)
            }
        }

        //  Ok(  formatted_address   )
    }

    pub async fn find_user_by_address(
        public_address: Address,
        psql_db: &Database,
    ) -> Result<User, PostgresModelError> {
        // let formatted_address = to_checksum(&public_address,None).to_string();

        let row = psql_db
            .query_one(
                "SELECT 
         id,   
         name,
         roles,
         public_address  
         FROM users 
         WHERE public_address = ($1::eth_address)
         LIMIT 1;",
                &[&DomainEthAddress(public_address)],
            )
            .await?;

        User::from_row(&row)
    }
     pub async fn find_by_id(
        user_id: i32,
        psql_db: &Database,
    ) -> Result<User, PostgresModelError> {
        // let formatted_address = to_checksum(&public_address,None).to_string();

        let row = psql_db
            .query_one(
                "SELECT 
            id,   
            name,
            roles,
            public_address  
            FROM users 
            WHERE id = ($1 )
            LIMIT 1;",
                &[&user_id],
            )
            .await?;

        User::from_row(&row)
    }

    pub async fn find_user_by_auth_token(
        auth_token: String,
        psql_db: &Database,
    ) -> Result<User, PostgresModelError> {
        let user_session =
           AccessTokensModel::find_authenticated_user_session(auth_token, psql_db).await?;

        //make sure auth token isnt expired ?
        let authenticated_user_public_address: Address = user_session.public_address;

        let authenticated_user =
            UsersModel::find_user_by_address(authenticated_user_public_address, psql_db).await?;

        Ok(authenticated_user)
    }
    
      pub async fn update_user(
        id: i32,
        
        name: Option<String>,
       
        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {

  
        
        psql_db.execute(
            
            "
            
             UPDATE users 
                SET 
                name = COALESCE($2, name) 
              
                
            WHERE id = $1;
            
            ",
            
             &[
                    &id, 
                    &name
                ],
        ).await?;
        
        

        Ok(id)
    }
    
}


 fn convert_roles_string_to_array(opt: Option<String>) -> Vec<String> {
      println!("opt {:?}", &opt);
      let roles = opt.unwrap_or_default();
    
  
    roles
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}