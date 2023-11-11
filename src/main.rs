use amqprs::{channel::{BasicConsumeArguments, BasicPublishArguments}, consumer::DefaultConsumer, BasicProperties};
use tokio::time;
use utils::constants::rabbit_mq_constants::{EXCHANGE, ROUTING_KEY};


use crate::db::tables::poll::{Poll, create_poll};

mod db;
mod utils;
mod server;

#[tokio::main]
async fn main() {
}
