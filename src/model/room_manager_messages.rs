use actix::prelude::*;

use crate::model::user::User;
use crate::ws::player_session::PlayerSession;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CreateWord {
    pub user: User,
    pub word: String,
    pub session_addr: Addr<PlayerSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomNextTurn {
    pub user: User,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomDamagePlayer {
    pub user: User,
    pub player_index: usize,
    pub damage: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub user: User,
    pub session_addr: Addr<PlayerSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomNextTurnTimeoutInit {
    pub user: User,
}