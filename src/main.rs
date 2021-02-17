use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth_play::{handlers, middleware::validate, postgres};
use deadpool_postgres::{tokio_postgres::NoTls, Client};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Creating a pool will always succeed, so if you want your application to crash on startup if no database
    // connection can be established just call db_pool.get().await right after creating the pool.
    let db_pool = auth_play::get_postgres_config().create_pool(NoTls).unwrap();
    let client: Client = db_pool.get().await.unwrap();
    client.batch_execute(postgres::CREATE_SCHEMA).await.unwrap();

    let server_addr = dotenv_codegen::dotenv!("SERVER_ADDR");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .data(web::JsonConfig::default().limit(4096)) // limit the size of the payload
            .service(web::resource("/auth").route(web::post().to(handlers::auth)))
            .service(
                web::resource("/validate")
                    .wrap(HttpAuthentication::bearer(validate))
                    .route(web::post().to(handlers::validate)),
            )
    })
    .bind(&server_addr)?
    .run();
    log::info!("Server running on {}", server_addr);

    server.await
}
