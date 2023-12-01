
/**
 * Internal crates
 */
use crate::models::user::user_model::{
    NewUserRequest, SessionInfo, SessionResponse, UserInfo, UserLoginRequest, UserResponse,
    UserToken,
};
/**
 * External crates
 */
extern crate bcrypt;
use actix_web::Error;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use uuid::Uuid;

/**
* Database
* */
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};

pub struct UserRepository;
impl UserRepository {
    pub async fn show(&self, pool: &Pool<AsyncPgConnection>) -> Result<UserResponse, Error> {
        //Use Database
        let mut conn = pool.get().await.unwrap();

        use crate::schema::tbl_users::dsl::*;

        let result: Vec<UserInfo> = tbl_users
            .select(UserInfo::as_select())
            .load::<UserInfo>(&mut conn)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error list users"))?;

        Ok(UserResponse { users: result })
    }

    pub async fn new_user(
        &self,
        pool: &Pool<AsyncPgConnection>,
        params: NewUserRequest,
    ) -> Result<NewUserRequest, Error> {
        let mut conn = pool.get().await.unwrap();
        use crate::schema::tbl_users::dsl::*;

        let new_user = diesel::insert_into(tbl_users)
            .values(&params)
            .execute(&mut conn)
            .await;

        match new_user {
            Ok(_) => Ok(params),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(
               err
            )),
        }
    }

    pub async fn is_valid_login_session(
        &self,
        user_token: UserToken,
        pool: &Pool<AsyncPgConnection>,
    ) -> Result<SessionResponse, Error> {
        //Use Database
        let mut conn = pool.get().await.unwrap();
        use crate::schema::tbl_users::dsl::*;

        let result_response: Vec<UserInfo> = tbl_users
            .filter(login_session.eq(user_token.login_session.clone()))
            .select(UserInfo::as_select())
            .load::<UserInfo>(&mut conn)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error"))?;

        Ok(SessionResponse {
            status: vec![SessionInfo {
                is_success: !result_response.is_empty(),
            }],
        })
    }

    pub async fn user_login(
        &self,
        user_params: &UserLoginRequest,
        pool: &Pool<AsyncPgConnection>,
    ) -> Result<UserResponse, Error> {
        let mut conn = pool.get().await.unwrap();
        use crate::schema::tbl_users::dsl::*;

        let mut query_result: Vec<UserInfo> = tbl_users
            .filter(
                user_name
                    .eq(&user_params.user_name.trim())
                    .and(password.eq(&user_params.password)),
            )
            .select(UserInfo::as_select())
            .load::<UserInfo>(&mut conn)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error"))?;

        if query_result.is_empty() {
            // No user found with the provided username and password
            return Err(actix_web::error::ErrorInternalServerError("Error"));
        } else {
            // Generate a new UUID for login_session
            let new_login_session = Uuid::new_v4().to_string();
            let userlogined_info: UserInfo = query_result[0].clone();

            let update_result = diesel::update(tbl_users.find(userlogined_info.id))
                .set(login_session.eq(&new_login_session))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(_) => {
                    query_result[0].login_session = Some(new_login_session);
                }
                Err(_err) => {
                    return Err(actix_web::error::ErrorInternalServerError("Error"));
                }
            }
        }

        Ok(UserResponse {
            users: query_result,
        })
    }
}
