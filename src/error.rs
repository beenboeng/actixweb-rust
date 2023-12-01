
use actix_web::{http::{StatusCode, header}, HttpResponse};
use serde::Serialize;
use std::fmt;

use crate::models::http::response::ResponseBody;


#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, status:bool, message: String,status_code:i64) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                status,
                message,
                status_code,
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.http_status);
        builder.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
        .json(&self.body)
    }
}

impl fmt::Display for ServiceError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "ServiceError: {}", self.body.message)
  }
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ShortErrorResponse {
  error: String,
}
#[allow(dead_code)]
impl ShortErrorResponse {
    pub fn new(error: &str) -> ShortErrorResponse {
      ShortErrorResponse {
        error: error.to_string()
      }
    }
}
