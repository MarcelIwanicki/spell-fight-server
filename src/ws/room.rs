use actix::Addr;

use crate::model::player_session_messages::{CheckWordExisting, DamagePlayer, NextTurn, StartPreparationTime, TakeDamage, WordCreated};
use crate::model::user::User;
use crate::util::constants::*;
use crate::ws::letters::get_random_letters;
use crate::ws::player_session::PlayerSession;

pub struct Room {
    pub users: Vec<User>,
    turn_of_player_index: u32,
    sessions: Vec<Addr<PlayerSession>>,
    max_players: usize,
}

impl Room {
    pub fn new(max_players: usize) -> Room {
        Room {
            sessions: Vec::new(),
            users: Vec::new(),
            turn_of_player_index: 0,
            max_players,
        }
    }

    pub fn is_player_turn(&self, player_session_addr: Addr<PlayerSession>) -> bool {
        let player_index = self.sessions.iter()
            .position(|session| &player_session_addr == session);
        let player_index = match player_index {
            Some(index) => index,
            None => return false,
        };
        let player_index = u32::try_from(player_index).unwrap_or(0);
        player_index == self.turn_of_player_index
    }

    pub fn increase_turn_index(&mut self) {
        let max_player = u32::try_from(self.max_players.clone()).unwrap_or(0);
        if self.turn_of_player_index.clone() < max_player {
            self.turn_of_player_index = self.turn_of_player_index.clone() + 1;
        } else {
            self.turn_of_player_index = 0;
        }

        for player in &self.sessions {
            let _ = player.do_send(NextTurn {
                player_index: usize::try_from(self.turn_of_player_index.clone()).unwrap_or(0),
                seconds: TURN_SECONDS,
            });
        }
    }

    pub fn add_player(&mut self, user: User, player_session_addr: Addr<PlayerSession>) {
        let is_player_already_in_room = self.sessions.contains(&player_session_addr);
        if self.sessions.len() < self.max_players && !is_player_already_in_room {
            self.sessions.push(player_session_addr);
            self.users.push(user);
        }
    }

    pub fn is_full(&self) -> bool {
        self.sessions.len() == self.max_players
    }

    pub fn start_game(&self) {
        for player in &self.sessions {
            let _ = player.do_send(StartPreparationTime {
                seconds: PREPARATION_TIME_SECONDS,
                users: self.users.clone(),
                letters: get_random_letters(MAX_LETTERS),
            });
        }
    }

    pub fn on_word_created(&self, word: String, player_session_addr: Addr<PlayerSession>) {
        let player_index = self.sessions.iter()
            .position(|session| &player_session_addr == session);

        let player_index = match player_index {
            Some(index) => index,
            None => return,
        };

        for player in &self.sessions {
            let word = word.clone();
            let player_index = player_index.clone();

            let _ = player.do_send(WordCreated {
                player_index,
                word,
            });
        }

        self.sessions[player_index.clone()].do_send(CheckWordExisting {
            player_index,
            word,
        })
    }

    pub fn on_damage_player(&self, damage: u32, player_index: usize) {
        self.sessions[player_index.clone()].do_send(TakeDamage { damage: damage.clone() });

        for player in &self.sessions {
            player.do_send(DamagePlayer {
                player_index: player_index.clone(),
                damage: damage.clone(),
            });
        }
    }
}