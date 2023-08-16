use std::time::Duration;

use actix::{Actor, Addr, AsyncContext, Handler, ResponseFuture, SpawnHandle, StreamHandler};
use actix_web_actors::ws;
use rand::seq::SliceRandom;

use crate::model::letter::Letter;
use crate::model::player_session_messages::{CanRollDice, CheckWordExisting, DamagePlayer, DiceRolled, NextTurn, StartPreparationTime, TakeDamage, WordCreated, WordDoesNotExist, WordExists};
use crate::model::room_manager_messages::{CreateWord, Join, RoomDamagePlayer, RoomNextTurn};
use crate::model::user::User;
use crate::model::ws_request::WsRequest;
use crate::model::ws_response::{DiceRolledResponse, WsResponse};
use crate::service::dictionary_service::DictionaryService;
use crate::util::constants::{MAX_PLAYERS_PER_ROOM, ROLL_DICE_SECONDS, TURN_SECONDS};
use crate::ws::letters::{get_random_letters, get_word_value};
use crate::ws::room_manager::RoomManager;

pub struct PlayerSession {
    pub player: User,
    pub health: u32,
    pub letters: Vec<Letter>,
    pub room_manager: Addr<RoomManager>,
    pub last_ws_response: Option<WsResponse>,
    pub last_word_exists: WordExists,
    pub roll_dice_timeout: Option<SpawnHandle>,
}

impl PlayerSession {
    pub fn new(player: User, room_manager: Addr<RoomManager>) -> PlayerSession {
        PlayerSession {
            player,
            health: 100,
            letters: Vec::new(),
            room_manager,
            last_ws_response: None,
            last_word_exists: WordExists {
                word: String::new(),
                damage: 0,
                player_index: 0,
            },
            roll_dice_timeout: None,
        }
    }
}

impl Actor for PlayerSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let request: Result<WsRequest, serde_json::Error> = serde_json::from_str(text.to_string().as_str());
                let request = match request {
                    Ok(req) => req,
                    Err(_) => return
                };

                match request {
                    WsRequest::Join => {
                        if let Some(_) = self.last_ws_response.clone() {
                            return;
                        }

                        self.room_manager.do_send(
                            Join {
                                user: self.player.clone(),
                                session_addr: ctx.address(),
                            }
                        );
                    }
                    WsRequest::CreateWord(word) => {
                        if let Some(ws_response) = self.last_ws_response.clone() {
                            if let WsResponse::NextTurn(_) = ws_response {
                                self.room_manager.do_send(
                                    CreateWord {
                                        user: self.player.clone(),
                                        session_addr: ctx.address(),
                                        word,
                                    }
                                )
                            }
                        }
                    }
                    WsRequest::RollDice => {
                        if let Some(ws_response) = self.last_ws_response.clone() {
                            if let WsResponse::CanRollDice(_) = ws_response {
                                ctx.address().do_send(
                                    DiceRolled {
                                        word_exists_event: self.last_word_exists.clone()
                                    }
                                );
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

impl Handler<NextTurn> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: NextTurn, ctx: &mut Self::Context) {
        if let Some(future) = self.roll_dice_timeout {
            ctx.cancel_future(future);
        }
        let next_turn_event = WsResponse::NextTurn(msg.clone());
        self.last_ws_response = Some(next_turn_event.clone());
        let next_turn_json = serde_json::to_string(&next_turn_event);
        let next_turn_json = match next_turn_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };
        let _ = ctx.text(next_turn_json);
    }
}

impl Handler<StartPreparationTime> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: StartPreparationTime, ctx: &mut Self::Context) {
        self.letters = msg.letters.clone();
        let start_preparation_time_event = WsResponse::StartPreparationTime(msg.clone());
        self.last_ws_response = Some(start_preparation_time_event.clone());
        let start_preparation_time_json = serde_json::to_string(&start_preparation_time_event);
        let start_preparation_time_json = match start_preparation_time_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };

        let _ = ctx.text(start_preparation_time_json);
        ctx.run_later(Duration::from_secs(msg.seconds), |_, ctx| {
            let start_game_event = NextTurn {
                player_index: 0,
                seconds: TURN_SECONDS,
            };

            let _ = ctx.address().do_send(start_game_event);
        });
    }
}

impl Handler<WordCreated> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: WordCreated, ctx: &mut Self::Context) {
        let word_created_event = WsResponse::WordCreated(msg.clone());
        self.last_ws_response = Some(word_created_event.clone());
        let word_created_json = serde_json::to_string(&word_created_event);
        let word_created_json = match word_created_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };
        let _ = ctx.text(word_created_json);
    }
}

impl Handler<CheckWordExisting> for PlayerSession {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: CheckWordExisting, ctx: &mut Self::Context) -> Self::Result {
        let address = ctx.address();
        let has_letters = player_has_letters_for_word(self.letters.clone(), msg.word.as_str());
        let future = async move {
            if word_exists(msg.word.as_str()).await && has_letters {
                let max_players = u32::try_from(MAX_PLAYERS_PER_ROOM).unwrap_or(0);
                let mut other_player_indices: Vec<u32> = (0..max_players).collect();
                other_player_indices.remove(msg.player_index);

                let player_index = (*other_player_indices.choose(&mut rand::thread_rng()).unwrap()).try_into().unwrap();

                address.do_send(WordExists {
                    word: msg.word.clone(),
                    damage: get_word_value(msg.word.clone()),
                    player_index,
                });
            } else {
                let word_not_found_event = WordDoesNotExist {};
                let _ = address.do_send(word_not_found_event);
            }
        };

        Box::pin(future)
    }
}

async fn word_exists(word: &str) -> bool {
    let dictionary_service = DictionaryService::new();
    dictionary_service.word_exists(word.to_ascii_lowercase().as_str()).await.unwrap_or(false)
}

fn player_has_letters_for_word(letters: Vec<Letter>, word: &str) -> bool {
    let mut letters_copy = letters.clone();
    for c in word.chars() {
        let found_letter_index = letters_copy.iter().position(|letter| {
            letter.letter.to_ascii_lowercase() == c.to_ascii_lowercase()
        });

        match found_letter_index {
            Some(index) => {
                letters_copy.remove(index);
            }
            None => {
                return false;
            }
        }
    }

    true
}

impl Handler<CanRollDice> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: CanRollDice, ctx: &mut Self::Context) {
        let can_roll_dice_message = WsResponse::CanRollDice(msg.clone());
        self.last_ws_response = Some(can_roll_dice_message.clone());
        let can_roll_dice_json = serde_json::to_string(&can_roll_dice_message);
        let can_roll_dice_json = match can_roll_dice_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };
        ctx.text(can_roll_dice_json);
    }
}

impl Handler<WordDoesNotExist> for PlayerSession {
    type Result = ();

    fn handle(&mut self, _msg: WordDoesNotExist, _ctx: &mut Self::Context) {
        self.room_manager.do_send(RoomNextTurn {
            user: self.player.clone()
        });
    }
}

impl Handler<WordExists> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: WordExists, ctx: &mut Self::Context) {
        self.last_word_exists = msg.clone();

        ctx.address().do_send(CanRollDice { seconds: ROLL_DICE_SECONDS });

        let roll_dice_timeout_future = move |_session: &mut PlayerSession, ctx: &mut Self::Context| {
            ctx.address().do_send(DiceRolled {
                word_exists_event: msg
            });
        };
        self.roll_dice_timeout = Some(
            ctx.run_later(Duration::from_secs(ROLL_DICE_SECONDS), roll_dice_timeout_future)
        );
    }
}

impl Handler<DiceRolled> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: DiceRolled, ctx: &mut Self::Context) {
        self.letters = remove_used_letters(self.letters.clone(), msg.word_exists_event.word.clone());
        self.letters = draw_missing_letters(self.letters.clone(), msg.word_exists_event.word.clone());

        let dice_rolled_message = WsResponse::DiceRolledResponse(DiceRolledResponse {
            amount: msg.word_exists_event.player_index.clone(),
            new_letters: self.letters.clone(),
        });
        self.last_ws_response = Some(dice_rolled_message.clone());
        let dice_rolled_json = serde_json::to_string(&dice_rolled_message);
        let dice_rolled_json = match dice_rolled_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };
        ctx.text(dice_rolled_json);

        self.room_manager.do_send(RoomDamagePlayer {
            user: self.player.clone(),
            damage: msg.word_exists_event.damage.clone(),
            player_index: msg.word_exists_event.player_index.clone(),
        });
        self.room_manager.do_send(RoomNextTurn {
            user: self.player.clone()
        });
    }
}

fn remove_used_letters(letters: Vec<Letter>, word: String) -> Vec<Letter> {
    let mut result = letters.clone();
    for c in word.chars() {
        if let Some(pos) = result.iter().position(|l| {
            l.letter.to_ascii_lowercase() == c.to_ascii_lowercase()
        }) {
            result.remove(pos);
        }
    }
    result
}

fn draw_missing_letters(letters: Vec<Letter>, word: String) -> Vec<Letter> {
    let mut result = letters.clone();
    let letters_count = letters.iter().count();
    let word_count = word.chars().count();
    let missing_letters_count = letters_count - word_count;
    let missing_letters = get_random_letters(missing_letters_count);
    missing_letters.iter().for_each(|l| result.push(l.clone()));
    result
}

impl Handler<DamagePlayer> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: DamagePlayer, ctx: &mut Self::Context) {
        let damage_player_message = WsResponse::DamagePlayer(msg.clone());
        self.last_ws_response = Some(damage_player_message.clone());
        let damage_player_json = serde_json::to_string(&damage_player_message);
        let damage_player_json = match damage_player_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };
        ctx.text(damage_player_json);
    }
}

impl Handler<TakeDamage> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: TakeDamage, ctx: &mut Self::Context) {
        self.health = self.health.clone() - msg.damage;

        let damage_player_message = WsResponse::TakeDamage(msg.clone());
        self.last_ws_response = Some(damage_player_message.clone());
        let damage_player_json = serde_json::to_string(&damage_player_message);
        let damage_player_json = match damage_player_json {
            Ok(json) => json,
            Err(_) => {
                return;
            }
        };
        ctx.text(damage_player_json);
    }
}