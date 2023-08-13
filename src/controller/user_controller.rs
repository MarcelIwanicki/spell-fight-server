use actix_web::{HttpResponse, Responder, web};

use crate::model::user::User;
use crate::repository::repository::Repository;
use crate::service::user_service::UserService;

pub async fn get_user<T: Repository<User>>(
    id: web::Path<String>,
    user_service: web::Data<UserService<T>>,
) -> impl Responder {
    if let Some(user) = user_service.get_user(&id.into_inner()).await {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

pub async fn create_user<T: Repository<User>>(
    user: web::Json<User>,
    user_service: web::Data<UserService<T>>,
) -> impl Responder {
    user_service.create_user(user.into_inner()).await;
    HttpResponse::Created().body("User created successfully")
}
