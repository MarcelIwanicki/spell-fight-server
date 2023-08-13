use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum WsRequest {
    Join,
    CreateWord(String),
    RollDice,
}