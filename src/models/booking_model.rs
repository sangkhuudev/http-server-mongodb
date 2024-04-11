use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, time::SystemTime};
use chrono::Utc;

use super::{dog_model::Dog, owner_model::Owner};
#[derive(Debug, Serialize, Deserialize)]
pub struct Booking {
    pub _id: ObjectId,
    pub owner: ObjectId,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub canceled: bool,
}

#[derive(Debug, Deserialize)]
pub struct BookingRequest {
    pub owner: String,
    pub start_time: String,
    pub duration_in_minutes: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBooking {
    pub _id: ObjectId,
    pub owner: Owner,
    dogs: Vec<Dog>,
    pub start_time: DateTime,
    pub duration_in_minutes: u8,
    pub canceled: bool,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;
    
    fn try_from(item: BookingRequest) -> Result<Self, Self::Error> {
        let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time)
            .map_err(|err| format!("Failed to parse time: {}", err))?
            .with_timezone(&Utc)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner"),
            start_time: DateTime::from(chrono_datetime),
            duration_in_minutes: item.duration_in_minutes,
            canceled: false,
        })
    }

}