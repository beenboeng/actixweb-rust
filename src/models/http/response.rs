use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub status: bool,     //The status is True/False  resonse from server api
    pub message: String,   //System description both success or failed
    pub status_code: i64,  //The NUMBER CODE standard implement on future the purpose API connect busines and business
    pub data: Option<T>,
}

impl<T> ResponseBody<T> {
    pub fn new(status: bool, message: &str, status_code: i64, data: Option<T>) -> ResponseBody<T> {
        ResponseBody {
            status,
            message: message.to_string(),
            status_code,
            data
        }
    }
}