use actix_web::Error;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::tbl_users;

#[derive(Queryable, Selectable, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = tbl_users)]
pub struct UserInfo{
    pub id              : i32,
    pub first_name      : Option<String>,
    pub last_name       : Option<String>,
    pub user_name       : Option<String>,
    #[serde(skip_serializing)]
    pub login_session   : Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserToken {
    // issued at
    pub iat             : i64,
    // expiration
    pub exp             : i64,
    // data
    pub id              : i32,
    pub user_name       : String,
    pub login_session   : String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct UserLoginInfo {
    pub id              : i32,
    pub user_name       : String,
    pub login_session   : String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionInfo{
    pub is_success      : bool,
}


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SessionResponse {
    pub status          : Vec<SessionInfo>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginedContext {
    // issued at
    pub iat             : i64,
    // expiration
    pub exp             : i64,
    // data
    pub id              : i32,
    pub user_name       : String,
    pub login_session   : String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct UserLoginRequest {
    pub user_name       : String,
    pub password        : String,
}

#[derive(Queryable, Selectable, Clone, Debug, PartialEq, Serialize, Deserialize,Insertable)]
#[diesel(table_name = tbl_users)]
pub struct NewUserRequest {
    pub first_name      : String,
    pub last_name       : String,
    pub user_name       : String,
    pub password        : String,
}


#[derive( Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub users           : Vec<UserInfo>
}

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
pub struct UserLoginResponse {
    pub token       : String,
    pub token_type  : String,
}



impl UserToken{
    pub fn new(iat: i64, exp: i64, id: i32, user_name: String ,login_session: String) -> Result<UserToken, String> {

        if iat.is_negative() {
            return Err(String::from("Iat cannot be negative"));
        }

        if exp.is_negative() {
            return Err(String::from("Expire cannot be negative"));
        }


        if user_name.is_empty() {
            return Err(String::from("User Name cannot be empty"));
        }

        if login_session.is_empty() {
            return Err(String::from("Login session cannot be empty"));
        }
        Ok(UserToken{iat, exp, id, user_name,login_session})
    }
}


impl UserResponse {
    pub fn get_users()-> Vec<UserInfo>{
        vec![
            UserInfo{
                id: 1,
                first_name: Some("Nana".to_string()),
                last_name: Some("Onikai".to_string()),
                user_name: Some("Nana Onikai".to_string()),
                login_session: Some("".to_string())
            },
            UserInfo{
                id: 2,
                first_name: Some("Lala".to_string()),
                last_name: Some("Onikai".to_string()),
                user_name: Some("Nana Onikai".to_string()),
               login_session: Some("".to_string())
            },
            UserInfo{
                id: 3,
                first_name: Some("Lulu".to_string()),
                last_name: Some("Onikai".to_string()),
                user_name: Some("Nana Onikai".to_string()),
               login_session: Some("".to_string())
            }
        ]
    }

    pub fn get_one_user(id:i32)-> Result<UserInfo, Error>{
        let users =  UserResponse::get_users();
        let result = users.iter().find(|&user| user.id == id);
        match result {
            Some(user) => Ok(user.clone()),
            None => Err(actix_web::error::ErrorBadRequest("User not found")),
        }
    }
}