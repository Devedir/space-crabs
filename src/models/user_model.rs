use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

// pub const USER_PASSWORD_SALT: &[u8] = b"nie wiem co ja tutaj robie";

#[derive(Debug, Serialize, Deserialize, Clone)] //generate implementation support for formatting the output, serializing, and deserializing the data structure.
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub login: String,
    pub password: String,
    pub role: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_expeditions: Option<Vec<MyExpedition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organized_expeditions: Option<Vec<MyExpedition>>,
    #[serde(skip_serializing_if = "Option::is_none")]

    pub contact: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyExpedition {
    pub exp_id: String,
    pub name: String,
    pub start_date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,
}

#[derive(FromForm)]
pub struct UserForm {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiUser {
    pub str_id: String,
    pub user: User
}

#[derive(Serialize, Deserialize)]
pub struct ContactOrganizator{
    pub expedition_name: String,
    pub organizer_name: String,
    pub contact: String
}
