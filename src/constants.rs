/*
    Static message for general usage 
*/


// Global Message 
#[allow(dead_code)]
pub const MESSAGE_OK: &str = "success";


#[allow(dead_code)]
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";

//Response message
#[allow(dead_code)]
pub const RESPONSE_OF_STATUS_IS_SUCCESS_TRUE: bool = true;
pub const RESPONSE_OF_STATUS_IS_SUCCESS_FALSE: bool = false;


#[allow(dead_code)]
pub const AUTH_CACHED_KEY_PREFIX_USER_NAME: &str = "auth:user_name:";//+ user_name

// ignore routes
#[allow(dead_code)]
pub const IGNORE_ROUTES: [&str; 2] = [
            "/api/ping", "/api/v1/web/users/login"
];


// Headers
#[allow(dead_code)]
pub const AUTHORIZATION: &str = "Authorization";
#[allow(dead_code)]
pub const SECWEBSOCKETPROTOCOL: &str = "Sec-WebSocket-Protocol";

