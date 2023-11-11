use actix_cors::Cors;
use actix_web::{HttpServer, App, dev::Server};

use super::api::get_polls_by_title;

/**
 * Creates a server struct, ready to be run
 */
pub fn create_server() -> Result<Server, std::io::Error> {
    Ok(
        HttpServer::new(|| {
            let cors = Cors::permissive();
    
            App::new()
                .wrap(cors)
                .service(get_polls_by_title)
        })
        .bind(("127.0.0.1", 6969))?
        .run()
    )
}