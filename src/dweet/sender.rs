use amqprs::{connection::{Connection, OpenConnectionArguments}, error::Error, channel::{Channel, QueueBindArguments, QueueDeclareArguments, BasicConsumeArguments}, consumer::DefaultConsumer};

use crate::utils::constants::rabbit_mq_constants::{URL, PORT, USER, PASSWORD, EXCHANGE, ROUTING_KEY};

/// Sends a message everytime a poll is posted to dweet.io

pub async fn establish_connection() -> Result<Connection, Error> {
    Connection::open(&OpenConnectionArguments::new(
        URL,
        PORT,
        USER,
        PASSWORD
    ))
    .await
}

pub async fn open_channel(conn: &Connection) -> Result<Channel, Error> {
    conn
        .open_channel(None)
        .await
}

pub async fn create_queue(channel: &Channel) -> Result<String, Error> {
    match channel
        .queue_declare(QueueDeclareArguments::default())
        .await {
            Ok(Some((queue_name, _, _))) => Ok(queue_name),
            Err(e) => Err(e),
            _ => Err(Error::NetworkError("No queue name recivied".to_owned()))
        }
}


pub async fn bind_queue(channel: &Channel, queue_name: &str) -> Result<(), Error> {
    channel
        .queue_bind(
            QueueBindArguments::new(queue_name, EXCHANGE, ROUTING_KEY)
        )
        .await
}
