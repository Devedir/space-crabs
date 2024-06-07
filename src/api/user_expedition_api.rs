use crate::{models::expedition_model::Expedition, repository::mongodb_repo::MongoRepo};
use mongodb:: results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

