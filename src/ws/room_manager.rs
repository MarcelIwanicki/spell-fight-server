use std::time::Duration;

use actix::{Actor, AsyncContext, Context, Handler};

use crate::model::room_manager_messages::{CreateWord, Join, RoomDamagePlayer, RoomNextTurn, RoomNextTurnTimeoutInit, RoomPlayerDead};
use crate::model::user::User;
use crate::util::constants::{MAX_PLAYERS_PER_ROOM, TURN_SECONDS};
use crate::ws::room::Room;

pub struct RoomManager {
    rooms: Vec<Room>,
}

impl RoomManager {
    pub fn new() -> RoomManager {
        RoomManager {
            rooms: vec![Room::new(MAX_PLAYERS_PER_ROOM)],
        }
    }

    fn find_room(&mut self, user: &User) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|room| room.users.contains(user))
    }
}

impl Actor for RoomManager {
    type Context = Context<Self>;
}

impl Handler<Join> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Context<Self>) {
        if let Some(room) = self.rooms.last_mut() {
            if room.is_full() {
                let mut new_room = Room::new(MAX_PLAYERS_PER_ROOM);
                new_room.add_player(msg.user, msg.session_addr);
                self.rooms.push(new_room);
            } else {
                room.add_player(msg.user, msg.session_addr);
                if room.is_full() {
                    room.start_game();
                }
            }
        }
    }
}

impl Handler<CreateWord> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: CreateWord, ctx: &mut Self::Context) {
        if let Some(room) = self.find_room(&msg.user) {
            let is_player_turn = room.is_player_turn(msg.session_addr.clone());
            if !is_player_turn {
                return;
            }

            if let Some(handle) = room.next_turn_timeout {
                ctx.cancel_future(handle);
                room.next_turn_timeout = None;
            }
            room.on_word_created(msg.word, msg.session_addr);
        }
    }
}

impl Handler<RoomNextTurn> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: RoomNextTurn, _ctx: &mut Self::Context) {
        if let Some(room) = self.find_room(&msg.user) {
            room.increase_turn_index();
        }
    }
}

impl Handler<RoomNextTurnTimeoutInit> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: RoomNextTurnTimeoutInit, ctx: &mut Self::Context) {
        if let Some(room) = self.find_room(&msg.user) {
            if let Some(_) = room.next_turn_timeout {
                return;
            }
            let roll_dice_timeout_future = move |_session: &mut RoomManager, ctx: &mut Self::Context| {
                ctx.address().do_send(RoomNextTurn {
                    user: msg.user.clone()
                });
            };
            room.next_turn_timeout = Some(
                ctx.run_later(Duration::from_secs(TURN_SECONDS), roll_dice_timeout_future)
            );
        }
    }
}

impl Handler<RoomDamagePlayer> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: RoomDamagePlayer, _ctx: &mut Self::Context) {
        if let Some(room) = self.find_room(&msg.user) {
            room.on_damage_player(msg.damage, msg.player_index);
        }
    }
}

impl Handler<RoomPlayerDead> for RoomManager {
    type Result = ();

    fn handle(&mut self, msg: RoomPlayerDead, _ctx: &mut Self::Context) {
        if let Some(room) = self.find_room(&msg.user) {
            room.on_player_dead(msg);
        }
    }
}