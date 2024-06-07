use serde::{Deserialize, Serialize};


pub enum ProcessorMessage {
    Success(String),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub article: String,
    pub author: String,
    pub paragraph: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paragraph {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: u32,
}