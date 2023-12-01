use crate::models::user::user_model::UserToken;

use jsonwebtoken::{DecodingKey, TokenData, Validation};

pub fn decode_user_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(b"token-secret-key007"),
        &Validation::default(),
    )
}
