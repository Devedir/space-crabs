use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult,UpdateResult,DeleteResult},
    sync::{Client, Collection},
};
use crate::models::expedition_model::Expedition;
use crate::models::user_model::User;

pub struct MongoRepo {
    expedition_col: Collection<Expedition>,
    user_col:Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("space-crabs");
        let expedition_col: Collection<Expedition> = db.collection("expeditions");
        let user_col:Collection<User> = db.collection("users");
        MongoRepo { expedition_col,user_col }
    }

    pub fn create_expedition(&self, new_expedition: Expedition) -> Result<InsertOneResult, Error> {

        let new_doc = Expedition {
            id:None, //mongoDB will create unique id
            name: new_expedition.name,
            stops: new_expedition.stops,
            max_no_participants: new_expedition.max_no_participants,
            guide: new_expedition.guide,
            organizer: new_expedition.organizer,
            start_time: new_expedition.start_time,
            end_time: new_expedition.end_time,
            home_station: new_expedition.home_station,
            participants: new_expedition.participants,
            price: new_expedition.price,
        };

        let expedition = self
            .expedition_col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating expedition");
        Ok(expedition)
    }

    pub fn get_expedition(&self, id: &String) -> Result<Expedition, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let expedition_detail = self
            .expedition_col
            .find_one(filter, None)
            .ok()
            .expect("Error getting expeditions's detail");
        Ok(expedition_detail.unwrap())
    }

//trzeba lepiej obsługiwać nested dokumenty

    // pub fn update_expedition(&self, id: &String, updated_expedition: Expedition) -> Result<UpdateResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let new_doc = doc! {
    //         "$set":
    //             {
    //                 "name": updated_expedition.name,
    //                 "stops": updated_expedition.stops,
    //                 "max_no_participants": updated_expedition.max_no_participants,
    //                 "guide": updated_expedition.guide,
    //                 "organizer": updated_expedition.organizer,
    //                 "start_time": updated_expedition.start_time,
    //                 "end_time": updated_expedition.end_time,
    //                 "home_station": updated_expedition.home_station,
    //                 "participants": updated_expedition.participants,
    //                 "price": updated_expedition.price,
    //             },
    //     };
    //     let updated_doc = self
    //         .col
    //         .update_one(filter, new_doc, None)
    //         .ok()
    //         .expect("Error updating expedition");
    //     Ok(updated_doc)
    // }

    pub fn delete_expedition(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let expedition_detail = self
            .expedition_col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting expedition");
        Ok(expedition_detail)
    }

    pub fn get_all_expeditions(&self) -> Result<Vec<Expedition>, Error> {
        let cursors = self
            .expedition_col
            .find(None, None)
            .ok()
            .expect("Error getting list of expeditions");
        let expeditions = cursors.map(|doc| doc.unwrap()).collect();
        Ok(expeditions)
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {

        let new_doc = User {
            id:None, //mongoDB will create unique id
            login: new_user.login,
            password:new_user.password,
            role:new_user.role,
            firstname:new_user.firstname,
            lastname:new_user.lastname,
            company_name:new_user.company_name,
            my_expeditions:new_user.my_expeditions,
            organized_expeditions:new_user.organized_expeditions,
            contact:new_user.contact,
        };

        let user = self
            .user_col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .user_col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let user = cursors.map(|doc| doc.unwrap()).collect();
        Ok(user)
    }
}