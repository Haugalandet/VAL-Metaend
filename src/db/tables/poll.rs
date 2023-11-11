use mongodb::{Client, bson::{DateTime, doc, Bson}, options::UpdateOptions};
use serde::{Serialize, Deserialize};
use crate::utils::constants::db_constants::{DB_NAME, DB_POLL_COLLECTION};

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub poll_id: Option<i32>,
    pub title: String,
    pub description: String,
    pub choices: Vec<Choice>,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub choice_id: Option<i32>,
    pub title: String,
    pub description: String,
    pub count: Option<i32>,
}

impl Into<Bson> for Choice {
    fn into(self) -> Bson {
        let mut doc = doc! {
            "title": self.title,
            "description": self.description,
        };

        if let Some(choice_id) = self.choice_id {
            doc.insert("choiceId", choice_id);
        }

        if let Some(count) = self.count {
            doc.insert("voteCount", count);
        }

        Bson::Document(doc)
    }
}



pub async fn create_poll(client: &Client, poll: Poll) -> mongodb::error::Result<()> {
    let collection = client.database(DB_NAME).collection(DB_POLL_COLLECTION);
    collection.insert_one(poll, None).await?;
    Ok(())
}


pub async fn find_polls_by_title(
    client: &Client,
    title: String,
) -> mongodb::error::Result<Vec<Poll>> {
    let collection = client.database(DB_NAME).collection::<Vec<Poll>>(DB_POLL_COLLECTION);

    let filter = doc! {
        "title": title,
    };

    let cursor = collection.find(filter, None).await?;


    cursor.deserialize_current()
}



pub async fn update_poll(client: &Client, poll: Poll) -> Result<(), mongodb::error::Error> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);

    let filter = doc! { "_id": poll.poll_id.clone() }; 
    let update = doc! {
        "$set": {
            "title": poll.title.clone(),
            "description": poll.description.clone(),
            "choices": poll.choices.clone(),
        },
    };

    let update_options = UpdateOptions::builder().upsert(false).build();

    // Perform the update operation.
    collection
        .update_one(filter, update, Some(update_options))
        .await?;

    Ok(())
}
