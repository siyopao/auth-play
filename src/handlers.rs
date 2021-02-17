use crate::{error::MyError, postgres};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use deadpool_postgres::{Client, Pool};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

const ISS: &str = "foobar corp.";

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    username: String,
    iss: String,
    exp: usize,
}

pub async fn auth(
    body: web::Json<User>,
    db_pool: web::Data<Pool>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::from)?;

    // Check if this user is okay to receive a token
    postgres::insert_user(&client, &body.username, &body.password).await?;

    let token = get_token(Claims {
        username: body.username.clone(),
        iss: ISS.to_string(),
        exp: (Utc::now() + Duration::hours(8)).timestamp() as usize,
    })?;

    Ok(HttpResponse::Ok().json(&AuthResponse { token }))
}

fn get_token(claims: Claims) -> Result<String, MyError> {
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(dotenv_codegen::dotenv!("API_SECRET").as_bytes()),
    )
    .map_err(MyError::from)
}

pub async fn validate() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("your token has been validated"))
}
