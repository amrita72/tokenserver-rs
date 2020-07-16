use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

use serde::{Deserialize, Serialize};
use std::convert::From;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token is Invalid")]
    InvalidToken,
    #[error("Issuer is Invalid")]
    InvalidIssuer,
    #[error("some other error")]
    Unknown,
}

// convert the error more appropriately.
impl From<jsonwebtoken::errors::Error> for TokenError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            ErrorKind::InvalidToken => TokenError::InvalidToken,
            ErrorKind::InvalidIssuer => TokenError::InvalidIssuer,
            _ => TokenError::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn verify_jwt_token(token: &str) -> Result<TokenData<Claims>, TokenError> {
    let pub_key = include_bytes!("public_rsa_key.pem");

    decode::<Claims>(
        &token,
        &DecodingKey::from_rsa_pem(pub_key).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .map_err(From::from)
}

pub fn generate_token(my_claims: &Claims) -> Result<String, TokenError> {
    let privkey_pem = include_bytes!("private_rsa_key.pem");

    encode(
        &Header::new(Algorithm::RS256),
        &my_claims,
        &EncodingKey::from_rsa_pem(privkey_pem).unwrap(),
    )
    .map_err(From::from)
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::Utc;

    #[test]
    fn test_token() {
        static THREE_DAYS: i64 = 60 * 60 * 24 * 3;
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; //nanoseconds -> seconds
        let my_claims = Claims {
            sub: "dummy_sub".to_string(),
            company: "dummy_company".to_string(),
            iat: now,
            exp: now + THREE_DAYS,
        };

        let token: String = generate_token(&my_claims).unwrap();
        println!("Original: {:?}", my_claims);
        match verify_jwt_token(&token) {
            Ok(value) => {
                println!(" decoded: {:?}", value.claims);
                assert!(value.claims == my_claims);
            }
            Err(err) => panic!("error: {}", err),
        };
    }
    #[test]
    fn test_invalid_token() {
        let token: String = String::from("bhxkgadweahfjhaweglfvawjcj");

        assert!(verify_jwt_token(&token).is_err());
    }
}
