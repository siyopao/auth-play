use crate::error::MyError;
use deadpool_postgres::{tokio_postgres::Row, Client};
use sha2::{Digest, Sha512};
use std::convert::TryFrom;

const INSERT_USER: &str = "INSERT INTO app_user (username, password) VALUES ($1, $2)";
const SELECT_USER: &str =
    "SELECT username, password FROM app_user WHERE username = $1 AND password = $2";

pub struct User {
    pub username: String,
    pub password: String,
}

impl TryFrom<Row> for User {
    type Error = MyError;

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            username: row.try_get("username").map_err(|_| MyError::RowError)?,
            password: row.try_get("password").map_err(|_| MyError::RowError)?,
        })
    }
}

pub async fn insert_user(client: &Client, username: &str, password: &str) -> Result<u64, MyError> {
    let password_hash = password_hash(password);
    let statement = client.prepare(&INSERT_USER).await?;
    client
        .execute(&statement, &[&username, &password_hash])
        .await
        .map_err(MyError::from)
}

pub async fn get_user(
    client: &Client,
    username: &str,
    password: &str,
) -> Result<Option<User>, MyError> {
    let password_hash = password_hash(password);
    let statement = client.prepare(&SELECT_USER).await?;

    client
        .query_opt(&statement, &[&username, &password_hash])
        .await?
        .map(User::try_from)
        .transpose()
}

fn password_hash(password: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
