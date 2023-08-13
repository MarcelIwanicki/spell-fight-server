use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;

use crate::model::oauth_callback_data::CallbackData;
use crate::model::user::User;
use crate::repository::mongo_db_user_repository::MongoDBUserRepository;
use crate::service::facebook_service::FacebookService;
use crate::service::user_service::UserService;

pub async fn facebook_callback(
    query: web::Query<CallbackData>,
    user_service: Data<UserService<MongoDBUserRepository>>,
    facebook_service: Data<FacebookService>,
) -> impl Responder {
    let code = query.code.clone();

    let access_token = facebook_service.get_facebook_access_token(&code).await;
    let access_token = match access_token {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };

    let facebook_profile = facebook_service.get_facebook_profile(&access_token).await;
    let facebook_profile = match facebook_profile {
        Ok(profile) => profile,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };

    let user = User::from_facebook_profile(facebook_profile);
    user_service.create_user(user).await;

    HttpResponse::Ok().json(access_token)
}