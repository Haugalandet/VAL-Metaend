use mongodb::{Client, bson::{DateTime, doc, Bson}, options::UpdateOptions, Collection};
use serde::{Serialize, Deserialize};
use crate::{utils::constants::db_constants::{DB_NAME, DB_POLL_COLLECTION}, db::establish_connection};
use futures::stream::TryStreamExt;
use mongodb::error::Error as MongoDBError;
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





pub async fn create_poll(client: &Client, poll: Poll) -> Result<(), MongoDBError> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);

    // Insert the poll into the collection
    if let Ok(result) = collection.insert_one(poll, None).await {
        if let Some(inserted_id) = result.inserted_id.as_object_id() {
            // Handle the inserted_id if needed
            println!("Poll inserted with ID: {:?}", inserted_id);
            Ok(())
        } else {
            // The insert was successful, but no inserted_id was returned
            Err(MongoDBError::custom("Insertion succeeded, but no inserted_id returned"))
        }
    } else {
        // Handle the error if the insertion failed
        Err(MongoDBError::custom("Failed to insert poll"))
    }
}



pub async fn find_polls_by_title(
    client: &Client,
    title: String,
) -> mongodb::error::Result<Vec<Poll>> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);

    let filter = doc! {
        "title": title,
    };

    let mut cursor = collection.find(filter, None).await?;

    let mut polls: Vec<Poll> = Vec::new();

    while let Some(p) = cursor.try_next().await? {
        polls.push(p);
    }

    Ok(polls)
}


pub async fn get_polls(
    client: &Client,
) -> mongodb::error::Result<Vec<Poll>> {
    let collection = client.database(DB_NAME).collection::<Poll>(DB_POLL_COLLECTION);


    let mut cursor = collection.find(None, None).await?;

    let mut polls: Vec<Poll> = Vec::new();

    while let Some(p) = cursor.try_next().await? {
        polls.push(p);
    }

    Ok(polls)
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


pub async fn create_test_polls() -> Result<(), mongodb::error::Error> {
    let conn = establish_connection().await?;

    create_poll(&conn, Poll {
        poll_id: None,
        title: "test".to_string(),
        description: "test desc".to_string(),
        choices: vec![],
    }).await?;
    
    Ok(())
}