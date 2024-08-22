use serde::{de, Serialize};

use crate::deserializer::RequestVisitor;

#[derive(Serialize, PartialEq, Debug)]
pub enum Request {
    Get { key: String },
    Set { key: String, val: String },
    Rm { key: String },
}

impl<'de> de::Deserialize<'de> for Request {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(RequestVisitor)
    }
}
