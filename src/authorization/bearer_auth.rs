use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::service::facebook_service::FacebookService;

pub async fn validate(request: ServiceRequest, bearer: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = bearer.token();
    let facebook_service = FacebookService::new();

    let facebook_profile = facebook_service.get_facebook_profile(&token).await;
    match facebook_profile {
        Ok(_) => Ok(request),
        Err(_) => {
            Err((actix_web::error::ErrorUnauthorized("Unauthorized"), request))
        }
    }
}