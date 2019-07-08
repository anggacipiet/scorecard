use actix_web::web;
use jwt::{decode, encode, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};
use derive_new::new;
use crate::model::{User, Token};
//use crate::errors::AppError;
use crate::db::{Pool, get_token};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

fn get_secret<'a>() -> &'a [u8] {
    dotenv!("JWT_SECRET").as_bytes()
}

pub fn create_token(user: User) -> String {
    let now = time::get_time().sec;
    let issued = Token {
        iat: now,
        exp: now + ONE_WEEK,
        user: user.username,
        uid: user.employee_id,
    };
    encode(&Header::default(), &issued, get_secret()).unwrap()
}

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<Token>> {
    decode::<Token>(&token, get_secret(), &Validation::default())
}

pub fn verify_token(token_data: &TokenData<Token>, pool: &web::Data<Pool>) -> Result<String, String> {
    let verify = get_token(&mut pool.get().unwrap(), &token_data.claims);
        match verify  {
            Ok(ok) =>  Ok(token_data.claims.user.to_string()),
            Err(_) => Ok("Invalid token".to_string()),
    }
}