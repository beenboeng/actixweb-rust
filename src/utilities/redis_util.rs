use actix_web::http::StatusCode;
use redis::{ToRedisArgs, FromRedisValue};
use crate::error::ServiceError;

#[allow(dead_code)]
pub async fn set<T>(
    key: &str,
    val: T,
    redis: &redis::Client,
) -> Result<bool, ServiceError>
where
    T: ToRedisArgs,
{
    let mut conn_redis = redis
            .get_tokio_connection_manager() 
            .await
            .map_err(|err| {
                ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    false,
                    format!("Redis connection error: {:?}", err),
                    1,
                )
            })?;
    let res = redis::Cmd::set(key, val)
                        .query_async::<_, String>(&mut conn_redis)
                        .await
                        .map_err(|err|
                            {
                                ServiceError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                false,
                                format!("Redis SET error: {:?}", err),
                                1,
                                )
                            }
                        )?;
    if res == "OK" {
        Ok(true)
    }else{
        Ok(false)
    }
    // println!("res: {:?}", res);
    
}


#[allow(dead_code)]
pub async fn get<T>(
    keys: &str,
    redis: &redis::Client,
) -> Result<Option<T>, ServiceError>
where
    T: ToRedisArgs + FromRedisValue, // add FromRedisValue trait
{
    let mut conn_redis = redis
            .get_tokio_connection_manager() 
            .await
            .map_err(|err| {
                ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    false,
                    format!("Redis connection error: {:?}", err),
                    1,
                )
            })?;
            let redis_response: Option<T> = redis::Cmd::get(keys).query_async(&mut conn_redis).await.map_err(|err|
                {
                    ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    false,
                    format!("Redis GET error: {:?}", err),
                    1,
                    )
                }
            )?;

            Ok(redis_response) 
    
}


#[allow(dead_code)]
pub async fn del(
    keys: &[&str],
    redis: &redis::Client,
) -> Result<bool, ServiceError>{
    let mut conn_redis = redis
            .get_tokio_connection_manager() 
            .await
            .map_err(|err| {
                ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    false,
                    format!("Redis connection error: {:?}", err),
                    1,
                )
            })?;
            let res = redis::Cmd::del(keys)
                        .query_async::<_, usize>(&mut conn_redis)
                        .await
                        .map_err(|err|
                            {
                                ServiceError::new(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                false,
                                format!("Redis Del error: {:?}", err),
                                1,
                                )
                            }
                        )?;
    if res > 0 {
        Ok(true)
    }else{
        Ok(false)
    }
    // println!("res: {:?}", res);
    
}

