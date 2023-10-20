/// Contains all files relevant to the database, including "entities" and their corresponding CRUD operations
/// 
/// Author: Nils Michael

pub mod tables;
use mongodb::{Client, options::ClientOptions};

/// Gets a possible connection with the database
pub async fn establish_connection() -> mongodb::error::Result<Client> {
    // Gets the connection
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Returns the connection
    Client::with_options(client_options)
}