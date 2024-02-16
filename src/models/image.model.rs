use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: Uuid,
    pub name: String,
    pub format: String,
    pub data: Vec<u8>,
}