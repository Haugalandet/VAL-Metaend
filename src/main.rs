use actix_cors::Cors;
use actix_web::{App, HttpServer};
use db::tables::poll::create_test_polls;
use server::api::{get_polls_by_title, get_all_polls};


mod db;
mod utils;
mod server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let r = create_test_polls().await;
    if r.is_err() {
        eprintln!("{:?}", r);
    }

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(get_polls_by_title)
            .service(get_all_polls)
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
