
use actix_web::web;

use crate::api::user_api;

pub fn users_routes(users_config: &mut web::ServiceConfig) {
    users_config.service(
        web::scope("users")
            .route("", web::get().to(user_api::show))
            .route("/new", web::post().to(user_api::new_user))
            .route("/login", web::post().to(user_api::user_login))
        );

}

