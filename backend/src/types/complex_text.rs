// helps with sanitization by disallowing semicolons and parenthesis
// which could be used to decimate our SQL if injected into queries
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

pub struct ComplexText(String);

impl ComplexText {
    pub fn new(s: &str) -> Option<Self> {
        Some(Self(s.into()))
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
}

impl Serialize for ComplexText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Simply serialize the inner string
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ComplexText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Ok(ComplexText(s))
    }
}
