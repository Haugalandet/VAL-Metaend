use serde::{Serialize, Deserialize};

use super::poll::Poll;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserTest {
    pub name: String,
    pub created_polls: Vec<Poll>,
    /// (Poll, Choice)
    pub voted_polls: Vec<(Poll, Vec<String>)>
}


