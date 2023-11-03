use mongodb::{Client, bson::{DateTime, doc}, options::UpdateOptions};
use serde::{Serialize, Deserialize};
use crate::utils::constants::db_constants::{DB_NAME, DB_POLL_COLLECTION};

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub id: i32,
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
    pub user: i32
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
            user: 0,
        }
    }
}


pub async fn create_poll(client: &Client, poll: Poll) -> mongodb::error::Result<()> {
    let collection = client.database(DB_NAME).collection(DB_POLL_COLLECTION);
    collection.insert_one(poll, None).await?;
    Ok(())
}


pub async fn find_polls_by_title_and_user(
    client: &Client,
    user: i32,
    title: String,
) -> mongodb::error::Result<Poll> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);

    let filter = doc! {
        "user": user,
        "title": title,
    };

    let cursor = collection.find(filter, None).await?;


    cursor.deserialize_current()
}



pub async fn update_poll(client: &Client, poll: Poll) -> Result<(), mongodb::error::Error> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);

    // Create a filter document to identify the poll you want to update, e.g., by its ID.
    let filter = doc! { "_id": poll.id.clone() }; // Assuming you have an ID field in your Poll struct.

    // Create an update document with the changes you want to apply to the poll.
    let update = doc! {
        "$set": {
            "title": poll.title.clone(), // Update the title with the new value from the Poll struct.
            // Add more fields to update as needed.
        },
    };

    // Set update options, if needed (e.g., to upsert if the poll doesn't exist).
    let update_options = UpdateOptions::builder().upsert(false).build();

    // Perform the update operation.
    collection
        .update_one(filter, update, Some(update_options))
        .await?;

    Ok(())
}
