use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::letter::Letter;
use crate::model::user::User;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NextTurn {
    pub player_index: usize,
    pub seconds: u64,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StartPreparationTime {
    pub seconds: u64,
    pub users: Vec<User>,
    pub letters: Vec<Letter>,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WordCreated {
    pub player_index: usize,
    pub word: String,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CheckWordExisting {
    pub player_index: usize,
    pub word: String,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CanRollDice {
    pub seconds: u64,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WordDoesNotExist;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct WordExists {
    pub word: String,
    pub player_index: usize,
    pub damage: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DiceRolled {
    pub word_exists_event: WordExists,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DamagePlayer {
    pub player_index: usize,
    pub damage: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TakeDamage {
    pub damage: u32,
}