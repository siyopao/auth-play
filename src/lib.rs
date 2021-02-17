#![feature(async_closure)]

pub mod error;
pub mod handlers;
pub mod middleware;
pub mod postgres;

/// The default pool `max_size` is set to `cpu_count * 4` ignoring any logical CPUs (Hyper-Threading).
pub fn get_postgres_config() -> deadpool_postgres::Config {
    deadpool_postgres::Config {
        host: Some(dotenv_codegen::dotenv!("POSTGRES.HOST").to_string()),
        port: Some(
            dotenv_codegen::dotenv!("POSTGRES.PORT")
                .parse::<u16>()
                .unwrap(),
        ),
        user: Some(dotenv_codegen::dotenv!("POSTGRES.USER").to_string()),
        password: Some(dotenv_codegen::dotenv!("POSTGRES.PASSWORD").to_string()),
        dbname: Some(dotenv_codegen::dotenv!("POSTGRES.DBNAME").to_string()),
        manager: Some(deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        }),
        ..Default::default()
    }
}
