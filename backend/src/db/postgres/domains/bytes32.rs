use bytes::BytesMut;

use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug)]
pub struct DomainBytes32(pub [u8; 32]);

impl<'a> FromSql<'a> for DomainBytes32 {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = <&str as FromSql>::from_sql(ty, raw)?;

        // Assuming you want to convert a hexadecimal string to bytes
        let bytes = hex::decode(s.trim_start_matches("0x"))?;

        if bytes.len() != 32 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected 32 bytes",
            )));
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);

        Ok(DomainBytes32(array))
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "bytes32"
    }
}

impl ToSql for DomainBytes32 {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let bytes_string = format!("0x{}", hex::encode(self.0));
        <&str as ToSql>::to_sql(&bytes_string.as_str(), ty, out)
    }

    fn accepts(sql_type: &Type) -> bool {
        sql_type.name() == "bytes32"
    }

    to_sql_checked!();
}
