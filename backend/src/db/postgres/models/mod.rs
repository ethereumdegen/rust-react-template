use serde::Serialize;


pub mod oauth_requests_model;

pub mod user_auth_tokens_model;
pub mod access_tokens_model;

pub mod users_model;

pub mod oauth_authorization_codes_model;

     
pub struct JoinQueryRow {
    key: String,
    name: String
}

impl JoinQueryRow {
    pub fn new(key:&str,name:&str) -> Self {
        Self {
           key: key.into(),
           name: name.into()
        }
    }
    
    pub fn to_string(&self) -> String {
        
        format!("{} AS {}",self.key,self.name)
        
    }
}
 
 
 pub fn expand_join_query_rows(rows: Vec<JoinQueryRow> ) -> String { 
    rows.iter()
   .map(|row| row.to_string())
   .collect::<Vec<_>>()
   .join(", ")
 }
 
 
#[derive(Serialize)]
pub struct RowsWithPaginationMetadata <T :  Serialize  > {
   
   
   rows: Vec<T>, 
   rows_count: i64 
   
}