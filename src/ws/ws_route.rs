use actix::Addr;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

use crate::model::player_session_messages::WordExists;
use crate::model::user::User;
use crate::service::facebook_service::FacebookService;
use crate::ws::player_session::PlayerSession;
use crate::ws::room_manager::RoomManager;

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    room_manager: web::Data<Addr<RoomManager>>,
    facebook_service: web::Data<FacebookService>,
) -> HttpResponse {
    let authorization_header = get_authorization_header(&req);
    let authorization_header = match authorization_header {
        Some(header) => header,
        None => { return HttpResponse::Unauthorized().finish(); }
    };
    let replaced = authorization_header.replace("Bearer ", "");
    let authorization_bearer = replaced.as_str();

    let facebook_profile = facebook_service.get_facebook_profile(authorization_bearer).await;
    let facebook_profile = match facebook_profile {
        Ok(profile) => profile,
        Err(_) => { return HttpResponse::Unauthorized().finish(); }
    };

    let player = User::from_facebook_profile(facebook_profile);

    let session = PlayerSession {
        player,
        health: 100,
        letters: Vec::new(),
        room_manager: room_manager.get_ref().clone(),
        last_word_exists: WordExists {
            word: String::new(),
            damage: 0,
            player_index: 0,
        },
        roll_dice_timeout: None,
    };

    let response = ws::start(session, &req, stream);
    let response = match response {
        Ok(res) => res,
        Err(_) => { return HttpResponse::Unauthorized().finish(); }
    };
    response
}

fn get_authorization_header(req: &HttpRequest) -> Option<&str> {
    req.headers().get("Authorization")?.to_str().ok()
}