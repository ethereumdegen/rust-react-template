use bytes::BytesMut;
use ethabi::ethereum_types::U256;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug)]
pub struct DomainUint256(pub U256);

impl<'a> FromSql<'a> for DomainUint256 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        /* let s = <&str as FromSql>::from_sql(ty, raw)?;

        let u256_val = U256::from_str_radix(s.trim_start_matches("0x"), 16)?;

        Ok(DomainUint256(u256_val))
        */
        println!("from sql 1");
        let s = <&str as FromSql>::from_sql(ty, raw)?;

        // Convert raw byte slice from varchar to a Rust String
        // let s: String = <String as FromSql>::from_sql(ty, raw)?;

        println!("from sql 2");

        // Now, you should parse this string into your DomainUint256 type
        // I'm not sure how your DomainUint256 type is structured, but here's a basic idea:
        let u256_value = U256::from_dec_str(&s)?; // Assuming a decimal string representation

        println!("from sql 3");

        Ok(DomainUint256(u256_value))
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "uint256"
    }
}

impl ToSql for DomainUint256 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        /*let uint_value = self.0.;
         <i64 as ToSql>::to_sql(&uint_value, ty, out)
        */

        let uint_string = self.0.to_string();
        println!("uint string {}", uint_string.clone());
        <&str as ToSql>::to_sql(&uint_string.as_str(), ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "uint256"
    }

    to_sql_checked!();
}
