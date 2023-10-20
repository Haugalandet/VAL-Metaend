use crate::db::{establish_connection, tables::{user::User, poll::{Poll, create_poll}}};

mod dweet;
mod db;
mod utils;

#[tokio::main]
async fn main() {

    if let Ok(client) = establish_connection().await {
    
        let poll = Poll {
            title: "Test Title".to_owned(),
            description: "Test Description".to_owned(),
            choice: vec!["Test Choice 1".to_owned(), "Test Choice 2".to_owned(), "Test Choice 3".to_owned()],
        };
    
        match create_poll(&client, poll).await {
            Ok(_) => {
                println!("Created example!");
            },
            Err(e) => {
                println!("Error: {}", e)
            }
        }
    } else {
        println!("Could not establish connection");
    }

}
