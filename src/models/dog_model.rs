use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Serialize)]
pub struct Dog {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DogRequest {
    pub owner: String,
    pub name: Option<String>,
    pub age: Option<u8>,
    pub breed: Option<String>,
}

impl TryFrom<DogRequest> for Dog {
    type Error = Box<dyn std::error::Error>;
    fn try_from(request: DogRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&request.owner).expect("Failed to parse owner"),
            name: request.name,
            age: request.age,
            breed: request.breed
        })
    }
}