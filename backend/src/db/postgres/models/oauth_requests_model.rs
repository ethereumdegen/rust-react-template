use degen_sql::db::postgres::postgres_db::Database;

use degen_sql::db::postgres::models::model::PostgresModelError;

use tokio_postgres::Row;

pub struct OAuthRequest {
    pub id: i32,
    pub state: String,
    pub code_verifier: String,
    pub domain_name: String,
}

impl OAuthRequest {
    pub fn from_joining_row(row: &Row) -> Result<Self, PostgresModelError> {
        let id: i32 = row.get("oauth_request_id");
        let state: String = row.get("oauth_request_state");
        let code_verifier: String = row.get("oauth_request_code_verifier");
        let domain_name: String = row.get("oauth_request_domain_name");

        Ok(Self {
            id,
            state,
            code_verifier,
            domain_name,
        })
    }

    pub fn get_select_query() -> String {
        " SELECT 
        oauth_request.id AS oauth_request_id,
        oauth_request.state AS oauth_request_state,
        oauth_request.code_verifier AS oauth_request_code_verifier,
        oauth_request.domain_name AS oauth_request_domain_name  
        

         "
        .into()
    }
}

pub struct OAuthRequestsModel {}

impl OAuthRequestsModel {
    pub async fn insert_one(
        state: String,
        code_verifier: String,
        domain_name: String,

        psql_db: &Database,
    ) -> Result<i32, PostgresModelError> {
        let insert_result = psql_db
            .query_one(
                "
            INSERT INTO oauth_requests 
            (
                state,
                code_verifier,
                domain_name 
                    
            ) 
            VALUES ($1, $2, $3 )
            RETURNING id;
            ",
                &[&state, &code_verifier, &domain_name],
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

    pub async fn find_by_state(
        state: String,

        psql_db: &Database,
    ) -> Result<OAuthRequest, PostgresModelError> {
        let row_result = psql_db
            .query_one(
                "
            SELECT  
            
                oauth_request.id AS oauth_request_id,
                oauth_request.state AS oauth_request_state,
                oauth_request.code_verifier AS oauth_request_code_verifier,
                oauth_request.domain_name AS oauth_request_domain_name 
                    
            
            FROM oauth_requests AS oauth_request 
            WHERE oauth_request.state = ($1)
            LIMIT 1 
            ;
            ",
                &[&state],
            )
            .await;

        match row_result {
            Ok(row) => Ok(OAuthRequest::from_joining_row(&row)?), // Successfully inserted new row and got its ID.
            Err(e) => {
                eprintln!("Database error: {:?}", e);

                Err(PostgresModelError::Postgres(e))
            }
        }
    }
}

/*




*/
