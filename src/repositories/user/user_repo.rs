use crate::constants;
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
use bcrypt::{hash, verify, DEFAULT_COST};
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

        let hashed_pass = match hash(params.password.clone(), DEFAULT_COST) {
            Ok(hashed) => hashed,
            Err(_) => actix_web::error::ErrorInternalServerError("Error Hash password").to_string(),
        };

        let new_user_data = NewUserRequest {
            first_name: params.first_name.to_owned(),
            last_name: params.last_name.to_owned(),
            user_name: params.last_name.to_owned(),
            password: hashed_pass,
        };

        let new_user = diesel::insert_into(tbl_users)
            .values(new_user_data)
            .execute(&mut conn)
            .await;

        match new_user {
            Ok(_) => Ok(params),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
        }
    }

    pub async fn is_valid_login_session(
        &self,
        user_token: UserToken,
        pool: &Pool<AsyncPgConnection>,
        redis: &redis::Client,
    ) -> Result<SessionResponse, Error> {
        //User redis Catch to chexk if catch exist

        // Redis Connection
        let mut redis_conn = redis
            .get_tokio_connection_manager()
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error redis connection!"))?;

        // Generate key catch
        let generate_catch_key = format!(
            "{}{}{}",
            constants::USER_CATCH_KEY,
            user_token.id,
            user_token.user_name
        );

        // redis::Cmd::set("test01", "u got it")
        // .query_async::<_, String>(&mut redis_conn)
        // .await
        // .map_err(|_| actix_web::error::ErrorInternalServerError("Error"))?;

        // Get the value from redis with key catch
        let redis_response = redis::Cmd::get(generate_catch_key)
            .query_async::<_, String>(&mut redis_conn)
            .await;

        // Get Result Catch as String
        let data_catch: Option<String> = match redis_response {
            Ok(res) => Some(res),
            Err(_) => None,
        };

        if !data_catch.is_none() {
            println!("ress")
        }

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
        redis: &redis::Client,
    ) -> Result<UserResponse, Error> {
        let mut conn = pool.get().await.unwrap();
        use crate::schema::tbl_users::dsl::*;

        let mut query_result: Vec<UserInfo> = tbl_users
            .filter(user_name.eq(&user_params.user_name.trim()))
            .select(UserInfo::as_select())
            .load::<UserInfo>(&mut conn)
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError("Error"))?;

        if query_result.is_empty() {
            // No user found with the provided username and password
            return Err(actix_web::error::ErrorUnauthorized("User does not exist"));
        } else {
            if match verify(
                user_params.password.clone(),
                &query_result
                    .get(0)
                    .unwrap()
                    .password
                    .clone()
                    .unwrap_or("".to_string()),
            ) {
                Ok(hashed) => hashed,
                Err(_) => false,
            } {
                return Err(actix_web::error::ErrorUnauthorized("Incorect password"));
            }

            // Generate a new UUID for login_session
            let new_login_session = Uuid::new_v4().to_string();
            let userlogined_info: UserInfo = query_result[0].clone();

            // Update user session
            let update_result = diesel::update(tbl_users.find(userlogined_info.id))
                .set(login_session.eq(&new_login_session))
                .execute(&mut conn)
                .await;

            match update_result {
                Ok(_) => {
                    //  Redis Connection
                    let mut redis_conn =
                        redis.get_tokio_connection_manager().await.map_err(|_| {
                            actix_web::error::ErrorInternalServerError("Error redis connection!")
                        })?;

                    // Generate key catch
                    let generate_catch_key = format!(
                        "{}{}{}",
                        constants::USER_CATCH_KEY,
                        userlogined_info.id,
                        userlogined_info.user_name.unwrap_or_default()
                    );

                    redis::Cmd::set(generate_catch_key, new_login_session.clone())
                        .query_async::<_, String>(&mut redis_conn)
                        .await
                        .map_err(|_| actix_web::error::ErrorInternalServerError("Error"))?;


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
