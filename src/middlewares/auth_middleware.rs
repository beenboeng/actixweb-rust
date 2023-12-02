use crate::{
    constants,
    models::{http::response::ResponseBody, user::user_model::UserLoginedContext},
    utilities::jwt_token_util,
};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use futures::future::LocalBoxFuture;
// use log::info;
// use log::info;
use crate::services::jwt::jwt_service::JwtService;

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    web::Data,
    Error, HttpMessage, HttpResponse,
};

pub struct Authentication;

impl<S: 'static, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        let mut authenticate_pass: bool = false;

        Box::pin(async move {
            let (reqs, _pl) = req.parts_mut();
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if reqs.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }

            if !authenticate_pass {
                if let Some(redis) = reqs.app_data::<Data<redis::Client>>() {
                    if let Some(pool) = reqs.app_data::<Data<Pool<AsyncPgConnection>>>() {
                        //info!("Connecting to database...");
                        if let Some(authen_header) = reqs.headers().get(constants::AUTHORIZATION) {
                            //info!("Parsing authorization header...");
                            if let Ok(authen_str) = authen_header.to_str() {
                                if authen_str.starts_with("bearer")
                                    || authen_str.starts_with("Bearer")
                                {
                                    // Get token by remove "bearer or Bearer"
                                    let token = authen_str[6..authen_str.len()].trim();

                                    // Start decode user token
                                    if let Ok(token_data) =
                                        jwt_token_util::decode_user_token(token.to_string())
                                    {
                                        let user_token = token_data.claims.clone();

                                        //info!("Decoding token...");
                                        let jwtvfyres = JwtService::verify_user_token(
                                            &user_token,
                                            pool.as_ref(),
                                            redis,
                                        )
                                        .await;

                                        match jwtvfyres {
                                            Ok(_jwtres) => {
                                                authenticate_pass = true;
                                            }
                                            Err(_error) => {
                                                authenticate_pass = false;
                                            }
                                        }
                                        let user_login_context = UserLoginedContext {
                                            iat: user_token.iat,
                                            exp: user_token.exp,
                                            id: user_token.id,
                                            user_name: user_token.user_name.clone(),
                                            login_session: user_token.login_session.clone(),
                                        };
                                        req.extensions_mut().insert(user_login_context);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !authenticate_pass {
                return Err(AuthError::Unauthorized.into());
            }

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}

use actix_web::ResponseError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            AuthError::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::Unauthorized => HttpResponse::Unauthorized()
                .append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .json(ResponseBody::new(
                    true,
                    constants::MESSAGE_INVALID_TOKEN,
                    -1,
                    Some(""),
                )),
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }
}
