use serde::{Deserialize, Serialize};

use crate::model::letter::Letter;
use crate::model::player_session_messages::{CanRollDice, DamagePlayer, NextTurn, PlayerDead, StartPreparationTime, TakeDamage, WordCreated};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum WsResponse {
    StartPreparationTime(StartPreparationTime),
    NextTurn(NextTurn),
    WordCreated(WordCreated),
    CanRollDice(CanRollDice),
    DiceRolledResponse(DiceRolledResponse),
    DamagePlayer(DamagePlayer),
    TakeDamage(TakeDamage),
    PlayerDead(PlayerDead),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiceRolledResponse {
    pub amount: usize,
    pub new_letters: Vec<Letter>,
}