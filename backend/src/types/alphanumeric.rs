// helps with sanitization by disallowing semicolons and parenthesis
// which could be used to decimate our SQL if injected into queries
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

//a flexible type !

#[derive(Debug, Clone)]
pub struct AlphaNumericString(String);

impl Serialize for AlphaNumericString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

struct AlphaNumericStringVisitor;

impl<'de> Visitor<'de> for AlphaNumericStringVisitor {
    type Value = AlphaNumericString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a number")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(AlphaNumericString(value.to_string()))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(AlphaNumericString(value.to_string()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(AlphaNumericString(value.to_string()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(AlphaNumericString(value.to_string()))
    }
}

impl<'de> Deserialize<'de> for AlphaNumericString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AlphaNumericStringVisitor)
    }
}

/*
pub struct AlphaNumericString(String);

use log::info;

impl AlphaNumericString {
    pub fn new(s: &str) -> Option<Self> {
        if s.chars().all(|c| char::is_ascii_alphanumeric(&c) || c.is_whitespace()) {
            Some(AlphaNumericString(s.into()))
        } else {
            None
        }
    }

    // Allow extraction of the inner String for uses that need it.
    pub fn inner(&self) -> &String {
        &self.0
    }

    pub fn to_string(&self) -> String {
        return self.0.clone();
    }

    pub fn to_i32(&self) -> Result<i32, std::num::ParseIntError> {
        i32::from_str(&self.0)
    }

     pub fn to_bool(&self) -> bool  {
        if self.0 == "on" || self.0 == "true" {
            return true
        }
        return false
    }
}

impl Serialize for AlphaNumericString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Simply serialize the inner string
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for AlphaNumericString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        return Self::new(&s)
        .ok_or_else(||  serde::de::Error::custom("String is not alphanumeric") )


    }
}
*/
