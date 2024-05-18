use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult,UpdateResult,DeleteResult},
    sync::{Client, Collection},
};
use crate::models::expedition_model::Expedition;

pub struct MongoRepo {
    col: Collection<Expedition>,
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
        let col: Collection<Expedition> = db.collection("expeditions");
        MongoRepo { col }
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
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating expedition");
        Ok(expedition)
    }

    pub fn get_expedition(&self, id: &String) -> Result<Expedition, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let expedition_detail = self
            .col
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
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting expedition");
        Ok(expedition_detail)
    }

    pub fn get_all_expeditions(&self) -> Result<Vec<Expedition>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of expeditions");
        let expeditions = cursors.map(|doc| doc.unwrap()).collect();
        Ok(expeditions)
    }
}