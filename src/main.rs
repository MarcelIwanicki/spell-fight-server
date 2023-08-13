use actix::Actor;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::authorization::bearer_auth::validate;
use crate::controller::{facebook_controller, user_controller};
use crate::repository::mongo_db_user_repository::MongoDBUserRepository;
use crate::service::facebook_service::FacebookService;
use crate::service::user_service::UserService;
use crate::ws::room_manager::RoomManager;
use crate::ws::ws_route::ws_route;

mod model;
mod controller;
mod repository;
mod service;
mod authorization;
mod env;
mod ws;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user_repository = MongoDBUserRepository::new().await.unwrap();
    let user_service = UserService::new(user_repository);
    let user_service = Data::new(user_service);

    let facebook_service = FacebookService::new();
    let facebook_service = Data::new(facebook_service);

    let room_manager = Data::new(RoomManager::new().start());

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validate);

        App::new()
            .app_data(user_service.clone())
            .app_data(facebook_service.clone())
            .app_data(room_manager.clone())
            .service(
                web::scope("/users")
                    .wrap(auth.clone())
                    .route("/{id}", web::get().to(user_controller::get_user::<MongoDBUserRepository>))
                    .route("", web::post().to(user_controller::create_user::<MongoDBUserRepository>))
            )
            .service(
                web::scope("/auth/facebook")
                    .route("/callback", web::get().to(facebook_controller::facebook_callback))
            )
            .service(
                web::scope("/ws")
                    .wrap(auth.clone())
                    .route("/", web::get().to(ws_route))
            )
    }).bind("127.0.0.1:8080")?.run().await
}