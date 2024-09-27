mod alert;
mod handlers;
mod middleware;
mod model;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use env_logger::Env;
use handlers::{event, get_transactions};
use model::{Record, Transaction, TxnType};
use surrealdb::{engine::local::Mem, Surreal};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(
        Env::default().default_filter_or("info"),
    );

    let db = Surreal::new::<Mem>(()).await?;

    db.use_ns("test").use_db("test").await?;

    HttpServer::new(move || {
        App::new()
            // .wrap(from_fn(event_mw))
            .app_data(Data::new(db.clone()))
            .service(event)
            .service(get_transactions)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
    .map_err(anyhow::Error::from)
}
