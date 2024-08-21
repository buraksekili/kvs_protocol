use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Request {
    Get { key: String },
    Set { key: String, val: String },
    Rm { key: String },
}
