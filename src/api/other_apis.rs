use std::collections::HashMap;

use crate::{
    models::{expedition_model::Expedition, user_model::{User, UserForm, ApiUser}},
    repository::mongodb_repo::MongoRepo
};
use rocket::{form::Form, http::{Cookie, CookieJar}};
use mongodb::{bson, results::{InsertOneResult, UpdateResult}};
use rocket::{
    http::Status, serde::json::Json, State
};
use rocket_dyn_templates::Template;
use rocket_dyn_templates::serde::json::json;

#[get("/account/<id>")]
pub fn get_account(
    db: &State<MongoRepo>,
    id: String
) -> Result<Template, Status> {

    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => {
            let mut context = HashMap::new();
            context.insert("user", user);
            Ok(Template::render("account", &context))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
