/**
 * External crates
 */
use actix_web::{
    Result, 
    http::StatusCode,
    Error
};

/**
* Database
**/
use diesel_async::{AsyncPgConnection, pooled_connection::deadpool::Pool};


/**
 * Internal crates
 */
use crate::{
    models::user::user_model::{UserResponse, UserToken, SessionResponse, UserLoginRequest, UserLoginInfo, UserInfo, NewUserRequest}, 
    repositories::user::user_repo::UserRepository, error::ServiceError, constants
};

pub struct UserServices;
impl UserServices {
    pub async fn show(&self,pool: &Pool<AsyncPgConnection> ) -> Result<UserResponse, Error> {
        UserRepository.show(&pool).await.map_err(Into::into)
    }

    pub async fn new_user(&self,pool: &Pool<AsyncPgConnection>, params:NewUserRequest)->Result<NewUserRequest,Error>{
        UserRepository.new_user(&pool, params).await.map_err(Into::into)
    }

    pub async fn check_session( &self, token_data: &UserToken,  pool: &Pool<AsyncPgConnection>) -> Result<SessionResponse, ServiceError> {

        //New instance userToken object from `model` & map_err to validate
        let user_token_model = UserToken::new(
            token_data.iat,
            token_data.exp,
            token_data.id,
            token_data.user_name.clone(),
            token_data.login_session.clone(),
        ).map_err(|_| {
            ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::RESPONSE_OF_STATUS_IS_SUCCESS_FALSE,
                constants::MESSAGE_INVALID_TOKEN.to_string(),
                1,
            )
        })?;

        
        //Add userToken `model` to repository
        let response = UserRepository.is_valid_login_session(user_token_model, pool).await;
        match response {
            Ok(session_info) => {
                //print!("session_info: {:#?}", session_info);
                if !session_info.status.is_empty() && session_info.status[0].is_success {
                    Ok(session_info)
                }else{
                    Err(ServiceError::new(
                        StatusCode::UNAUTHORIZED,
                        constants::RESPONSE_OF_STATUS_IS_SUCCESS_FALSE,
                        constants::MESSAGE_INVALID_TOKEN.to_string(),
                        1,
                    ))
                }
            },
            Err(_) => Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::RESPONSE_OF_STATUS_IS_SUCCESS_FALSE,
                constants::MESSAGE_INVALID_TOKEN.to_string(),
                1,
            )),
        }
    }

    pub async fn user_login(
        &self,
        user_params: UserLoginRequest,
        pool: &Pool<AsyncPgConnection>
    ) -> Result<UserLoginInfo, Error> {
        let response: std::prelude::v1::Result<_, _> = UserRepository.user_login(&user_params, pool).await;
        match response {
            Ok(auth_users) => {
           
                let user_authenticated: UserInfo = auth_users.users[0].clone();

                let user_logined_info: UserLoginInfo = UserLoginInfo{
                    id: user_authenticated.id,
                    user_name: user_authenticated.user_name.unwrap(),
                    login_session:user_authenticated.login_session.unwrap(),
                };

                
                Ok(user_logined_info)
            },
            Err(_) => Err(actix_web::error::ErrorInternalServerError("user service")),
        }

    }
    
}