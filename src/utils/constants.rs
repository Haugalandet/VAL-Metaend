pub mod db_constants {
    pub const DB_NAME: &str = "val_db";
    pub const DB_POLL_COLLECTION: &str = "poll";
}

pub mod rabbit_mq_constants {
    pub const URL: &str = "localhost";
    pub const PORT: u16 = 5672;
    pub const USER: &str = "guest";
    pub const PASSWORD: &str = "guest";
    pub const EXCHANGE: &str = "amq.topic";
    pub const ROUTING_KEY: &str = "amqprs.example";
}