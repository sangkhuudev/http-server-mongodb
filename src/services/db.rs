use chrono::Utc;
use futures_util::stream::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, from_document, oid::ObjectId, DateTime},
    results::{InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::{str::FromStr, time::SystemTime};
use crate::models::{booking_model::{Booking, FullBooking}, dog_model::Dog, owner_model::Owner};

pub struct Database {
    pub booking: Collection<Booking>,
    pub owner: Collection<Owner>,
    pub dog: Collection<Dog>,
}

impl Database {
    pub async fn init() -> Self {
        let uri = match std::env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };
        let client = Client::with_uri_str(&uri).await.unwrap();
        let db = client.database("dog_walking");
        let booking: Collection<Booking> = db.collection("booking");
        let owner: Collection<Owner> = db.collection("owner");
        let dog: Collection<Dog> = db.collection("dog");

        Self {
            booking,
            owner,
            dog
        }
    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
        let result = self
            .owner
            .insert_one(owner, None)
            .await
            .ok()
            .expect("Error creating owner");

        Ok(result)
    }

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
        let result = self
            .dog
            .insert_one(dog, None)
            .await
            .ok()
            .expect("Error creating dog");

        Ok(result)
    }

    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
        let result = self
            .booking
            .insert_one(booking, None)
            .await
            .ok()
            .expect("Error creating booking");

        Ok(result)
    }

    pub async fn cancle_booking(&self, booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
            .booking
            .update_one(
                doc! {
                    "_id": ObjectId::from_str(booking_id).expect("Error parsing booking_id")
                },
                doc!{
                    "$set": doc!("canceled": true)
                },
                None
            )
            .await
            .ok()
            .expect("Error cancle booking");

        Ok(result)
    }

    pub async fn get_bookings(&self) -> Result<Vec<FullBooking>, Error> {
        let now: SystemTime = Utc::now().into();
    
        let mut results = self
            .booking
            .aggregate(
                vec![
                    doc! {
                        "$match": {
                            "cancelled": false,
                            "start_time": {
                                "$gte": DateTime::from_system_time(now)
                            }
                        }
                    },
                    doc! {
                        "$lookup": doc! {
                            "from": "owner",
                            "localField": "owner",
                            "foreignField": "_id",
                            "as": "owner"
                        }
                    },
                    doc! {
                        "$unwind": doc! {
                            "path": "$owner"
                        }
                    },
                    doc! {
                        "$lookup": doc! {
                            "from": "dog",
                            "localField": "owner._id",
                            "foreignField": "owner",
                            "as": "dogs"
                        }
                    },
                ],
                None,
            )
            .await
            .ok()
            .expect("Error getting bookings");

        let mut bookings: Vec<FullBooking> = Vec::new();

        while let Some(result) = results.next().await {
            match result {
                Ok(res) => {
                    let booking: FullBooking = from_document(res).expect("Error converting document to FullBooking");
                    bookings.push(booking);
                },
                Err(err) => panic!("Error getting booking: {}", err)
            }
        }

        Ok(bookings)
    }

}