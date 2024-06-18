use std::collections::HashMap;

use crate::{
    models::expedition_model::{Expedition, ApiExpedition},
    repository::mongodb_repo::MongoRepo
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use rocket_dyn_templates::Template;

#[post("/expedition", data = "<new_expedition>")]
pub fn create_expedition(
    db: &State<MongoRepo>,
    new_expedition: Json<Expedition>,
) -> Result<Json<InsertOneResult>, Status> {

    let expedition: Expedition = new_expedition.into_inner(); //change from Json<Expedition> to Expedition
    let expedition_detail = db.create_expedition(expedition);
    let result = match expedition_detail {
        Ok(expedition) => Ok(Json(expedition)),
        Err(_) => Err(Status::InternalServerError),
    };

    db.add_expedition_to_organizator(
        expedition.organizer.expect("Organiser not provided!").org_id,
        expedition
    );

    result
}


#[get("/expedition/<path>")]
pub fn get_expedition(
    db: &State<MongoRepo>,
    path: String
) -> Result<Template, Status> {

    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let expedition_detail = db.get_expedition(&id);
    match expedition_detail {
        Ok(expedition) => {
            let mut context = HashMap::new();
            context.insert("expedition", expedition);
            Ok(Template::render("expedition", &context))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/expedition/<path>", data = "<new_expedition>")]
pub fn update_expedition(
    db: &State<MongoRepo>,
    path: String,
    new_expedition: Json<Expedition>,
) -> Result<Json<Expedition>, Status> {

    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data: Expedition = new_expedition.into_inner();
    let update_result = db.update_expedition(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_expedition_info = db.get_expedition(&id);
                return match updated_expedition_info {
                    Ok(expedition) => Ok(Json(expedition)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/expedition/<path>")]
pub fn delete_expedition(
    db: &State<MongoRepo>,
    path: String
) -> Result<Json<&str>, Status> {
    
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_expedition(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Expedition successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/expeditions")]
pub fn get_all_expeditions(db: &State<MongoRepo>) -> Result<Template, Status> {
    let maybe_expeditions = db.get_all_expeditions();
    match maybe_expeditions {
        Ok(expeditions) => {
            let api_expeditions: Vec<ApiExpedition> = expeditions.iter()
                .map(|exp| ApiExpedition {
                    str_id: exp.id.unwrap().to_hex(),
                    expedition: exp.clone()
                }).collect();
            let mut context = HashMap::new();
            context.insert("api_expeditions", api_expeditions);
            Ok(Template::render("expeditions", &context))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/expedition/<path>", data = "<data>")]
pub fn add_expedition_to_user(
    db: &State<MongoRepo>,
    path: String,
    data: String
) -> Result<Json<Expedition>, Status> {

    let expedition_id = path;
    let user_id = data;
    let result = db.add_expedition_to_user(&user_id,&expedition_id);
    match result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_expedition_info = db.get_expedition(&expedition_id);
                return match updated_expedition_info {
                    Ok(expedition) => Ok(Json(expedition)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
