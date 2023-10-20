use mongodb::{Client, bson::DateTime};
use serde::{Serialize, Deserialize};
use crate::utils::constants::db_constants::{DB_NAME, DB_POLL_COLLECTION};

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub title: String,
    pub description: String,
    pub choice: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PollTest {
    pub instances: usize,
    /// ([Title], Instance id)
    pub title: Vec<(String, usize)>,
    /// ([Description], Instance id)
    pub description: Vec<(String, usize)>,
    pub choices: Vec<String>,
    /// ([(Choice, Count)], instance id)
    pub votes: Vec<(Vec<(String, u32)>, usize)>,
    /// (Possible IOT device, instance id)
    pub iot_device: Vec<(Option<String>, usize)>,
    pub creation_date: DateTime,
    /// (Opened, Closed, Instance)
    pub dates: Vec<(DateTime, DateTime, usize)>,
}

impl PollTest {
    pub fn new(title: String, description: String, choices: Vec<String>, votes: Vec<(String, u32)>, iot_device: Option<String>, creation_date: DateTime) -> PollTest {
        PollTest {
            instances: 0,
            title: vec![(title, 0)],
            description: vec![(description, 0)],
            choices,
            votes: vec![(votes, 0)],
            iot_device: vec![(iot_device, 0)],
            creation_date,
            dates: Vec::new(),
        }
    }
}


pub async fn create_poll(client: &Client, poll: Poll) -> mongodb::error::Result<()> {
    let collection = client.database(DB_NAME).collection(DB_POLL_COLLECTION);
    collection.insert_one(poll, None).await?;
    Ok(())
}


pub async fn find_polls(client: &Client) -> mongodb::error::Result<Vec<Poll>> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);
    let mut cursor = collection.find(None, None).await?;
    
    let mut res = Vec::new();

    while let Ok(p) = cursor.deserialize_current() {
        res.push(p);
        cursor.advance().await?;
    }

    Ok(res)
}