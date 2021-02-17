use crate::{error::MyError, handlers::Claims};
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub async fn validate(req: ServiceRequest, bearer: BearerAuth) -> Result<ServiceRequest, Error> {
    // Adding some leeway (in seconds) for exp and nbf checks
    let validation = Validation {
        algorithms: vec![Algorithm::HS512],
        leeway: 60,
        ..Default::default()
    };

    decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(dotenv_codegen::dotenv!("API_SECRET").as_bytes()),
        &validation,
    )
    .map(|_| req)
    .map_err(|e| MyError::from(e).into())
}
