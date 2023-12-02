/**
 * External crates
 */
use actix_web::{web, HttpResponse, Result};
use diesel_async::pooled_connection::deadpool::Pool;

/**
 * Internal crates
 */
use crate::{
    constants,
    models::{
        http::response::ResponseBody,
        user::{
            self,
            user_model::{NewUserRequest, UserLoginRequest, UserLoginResponse},
        },
    },
    services::{jwt::jwt_service::JwtService, user::user_services::UserServices},
};

/**
 * DB
 **/
use diesel_async::AsyncPgConnection;

pub async fn show(pool: web::Data<Pool<AsyncPgConnection>>) -> Result<HttpResponse> {
    let response = UserServices.show(pool.as_ref()).await;
    match response {
        Ok(users) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            true,
            constants::MESSAGE_OK,
            1000,
            Some(users),
        ))),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}

pub async fn new_user(
    pool: web::Data<Pool<AsyncPgConnection>>,
    params: web::Json<NewUserRequest>,
) -> Result<HttpResponse> {
    let response = UserServices
        .new_user(pool.as_ref(), params.into_inner())
        .await;
    match response {
        Ok(users) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            true,
            constants::MESSAGE_OK,
            1000,
            Some(users),
        ))),
        Err(err) => Ok(HttpResponse::Ok().body(err.to_string())),
    }
}

pub async fn user_login(
    user_params: web::Json<UserLoginRequest>,
    pool: web::Data<Pool<AsyncPgConnection>>,
    redis: web::Data<redis::Client>,
) -> Result<HttpResponse> {
    let auth_response = UserServices
        .user_login(user_params.into_inner(), pool.as_ref(), &redis)
        .await;

    match auth_response {
        Ok(user) => {
            let token = JwtService::generate_user_token(&user);

            let auth_login_response = UserLoginResponse {
                token,
                token_type: "bearer".to_string(),
            };

            Ok(HttpResponse::Ok().json(ResponseBody::new(
                true,
                "login success",
                1,
                Some(auth_login_response),
            )))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().json(ResponseBody::new(
            false,
            "Wrong password",
            401,
            Some({}),
        ))),
    }
}

pub async fn kill_user_session(
    user_id: web::Path<i32>,
    pool: web::Data<Pool<AsyncPgConnection>>,
    redis: web::Data<redis::Client>,
) -> Result<HttpResponse> {
    let response = UserServices
        .kill_user_session(user_id.into_inner(), pool.as_ref(), &redis)
        .await;

    match response {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(
            true,
            "Done kick user",
            1,
            Some(user),
        ))),
        Err(_) => Ok(HttpResponse::Unauthorized().json(ResponseBody::new(
            false,
            "Wrong password",
            401,
            Some({}),
        ))),
    }
}
