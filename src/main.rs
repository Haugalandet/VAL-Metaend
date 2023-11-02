use amqprs::{channel::{BasicConsumeArguments, BasicPublishArguments}, consumer::DefaultConsumer, BasicProperties};
use dweet::sender::{establish_connection, open_channel, create_queue, bind_queue};
use tokio::time;
use utils::constants::rabbit_mq_constants::{EXCHANGE, ROUTING_KEY};

use crate::db::tables::{user::User, poll::{Poll, create_poll}};

mod dweet;
mod db;
mod utils;

#[tokio::main]
async fn main() {
    let _ = test_rabbitmq().await;
}

async fn test_mongodb() {
    if let Ok(client) = db::establish_connection().await {
    
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

async fn test_rabbitmq() -> Result<(), amqprs::error::Error> {
    let conn = establish_connection().await?;

    let channel = open_channel(&conn).await?;

    let queue_name = create_queue(&channel).await?;

    bind_queue(&channel, &queue_name).await?;

    //////////////////////////////////////////////////////////////////
    // start consumer with given name
    let args = BasicConsumeArguments::new(
        &queue_name,
        "example_basic_pub_sub"
    );

    channel
        .basic_consume(DefaultConsumer::new(args.no_ack), args)
        .await?;

    //////////////////////////////////////////////////////////////////
    // publish message
    let content = String::from(
        r#"
            {
                "publisher": "example"
                "data": "Hello, amqprs!"
            }
        "#,
        )
        .into_bytes();

    // create arguments for basic_publish
    let args = BasicPublishArguments::new(EXCHANGE, ROUTING_KEY);

    channel
        .basic_publish(BasicProperties::default(), content, args)
        .await?;


    // channel/connection will be closed when drop.
    // keep the `channel` and `connection` object from dropping
    // before pub/sub is done.
    time::sleep(time::Duration::from_secs(1)).await;
    // explicitly close
    channel.close().await.unwrap();
    conn.close().await.unwrap();

    Ok(())
}