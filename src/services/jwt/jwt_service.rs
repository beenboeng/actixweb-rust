/**
 * External Crates
 **/
use actix_web::http::StatusCode;
use chrono::Utc;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Serialize, Deserialize};

/**
 * Internal Crates
 **/
use crate::{
    models::user::user_model::{
                UserLoginInfo,
                UserToken
    },
    error::ServiceError,
    constants, services::user::user_services::UserServices,
};


pub static KEY: [u8; 16] = *include_bytes!("../../secret.key");
static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtService;

impl JwtService {

    pub fn generate_user_token(login: &UserLoginInfo) -> String {
        let now = Utc::now().timestamp_nanos_opt().unwrap() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            id: login.id,
            user_name: login.user_name.clone(),
            login_session: login.login_session.clone(),
            
        };

        jsonwebtoken::encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(b"token-secret-key007"),
        )
        .unwrap()
    }

    pub async fn verify_user_token(
        token_data: &UserToken,
        pool: &Pool<AsyncPgConnection>,
        redis:&redis::Client
    ) -> Result<i32, ServiceError> {

        let response =  UserServices.check_session(
            token_data,
            pool,
            redis
        ).await;
       
        match response {
            Ok(_userinfo) => Ok(
                token_data.id
            ),
            Err(_) => Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::RESPONSE_OF_STATUS_IS_SUCCESS_FALSE,
                constants::MESSAGE_INVALID_TOKEN.to_string(),
                1,
            )),
        }
    }
}
