use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)] //generate implementation support for formatting the output, serializing, and deserializing the data structure.
pub struct Expedition {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub stops: Vec<Stop>,
    pub max_no_participants: i64,
    pub guide: Guide,
    pub organizer: Organizer,
    pub start_time: i64,
    pub end_time: i64,
    pub home_station: String,
    pub participants: Vec<Participant>,
    pub price: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub name: String,
    pub arrival_time: i64,
    pub departure_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Guide {
    pub firstname: String,
    pub lastname: String,
    pub age: i64,
    pub experience: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Organizer {
    #[serde(rename = "org_id")]
    pub org_id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: String,
    pub firstname: String,
    pub lastname: String,
    pub paid: bool,
}
