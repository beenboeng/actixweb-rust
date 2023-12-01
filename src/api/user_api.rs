/**
 * External crates
 */
use actix_web::{HttpResponse,Result, web};
use diesel_async::pooled_connection::deadpool::Pool;

/**
 * Internal crates
 */
use crate::{
    services::{user::user_services::UserServices, jwt::jwt_service::JwtService}, 
    models::{http::response::ResponseBody, user::user_model::{UserLoginRequest, UserLoginResponse, NewUserRequest}}, constants
};

/**
 * DB
 **/
 use diesel_async::AsyncPgConnection;


pub async fn show(pool: web::Data<Pool<AsyncPgConnection>>) -> Result<HttpResponse> {
    let response = UserServices.show(pool.as_ref()).await;
    match response {
        Ok(users) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(true, constants::MESSAGE_OK, 1000, users)
        )),
        Err(_) => Ok(HttpResponse::Ok().body("Error")),
    }
}

pub async fn new_user(pool: web::Data<Pool<AsyncPgConnection>>,params: web::Json<NewUserRequest>)-> Result<HttpResponse>{

    let response = UserServices.new_user(pool.as_ref(), params.into_inner()).await;
    match response {
        Ok(users) => Ok(HttpResponse::Ok().json(
            ResponseBody::new(true, constants::MESSAGE_OK, 1000, users)
        )),
        Err(err) => Ok(HttpResponse::Ok().body(err.to_string())),
    }
}


pub async fn user_login(
    user_params: web::Json<UserLoginRequest>,
    pool: web::Data<Pool<AsyncPgConnection>>
) -> Result<HttpResponse> {
    let auth_response = UserServices.user_login(user_params.into_inner(), pool.as_ref()).await;

    match auth_response {
        Ok(user) => {
            let token = JwtService::generate_user_token(&user);

            let auth_login_response = UserLoginResponse{
                token,
                token_type  : "bearer".to_string(),
            };

            Ok(
                HttpResponse::Ok().json(
                    ResponseBody::new(
                        true,
                        "login success",
                        1,
                        auth_login_response
                    )
                )
            )
        }
        Err(err) => Ok(err.error_response()),
    }

}