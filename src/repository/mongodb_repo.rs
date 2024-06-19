use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{ doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use crate::models::{expedition_model::Expedition, user_model::ContactOrganizator};
use crate::models::user_model::User;


pub struct MongoRepo {
    pub expedition_col: Collection<Expedition>,
    pub user_col: Collection<User>
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
            id: None, //mongoDB will create unique id
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


    pub fn add_expedition_to_organizator(
        &self,
        user_id: &String,
        // mut new_expedition: Expedition
        new_expedition: Expedition
    ) -> Result<UpdateResult, Error> {

        // if new_expedition.organizer == None {
        //     let user_detail = match self.get_user(user_id) {
        //         Ok(user) => user,
        //         Err(err) => return Err(err),
        //     };
        //
        //     let expedition_organizer = Organizer {
        //         name: user_detail.company_name.expect("Not an organiser!"),
        //         org_id: user_id.to_string(),
        //     };
        //     new_expedition.organizer = Some(expedition_organizer);
        // }

        let expedition_id = match self.create_expedition(new_expedition.clone()){
            Ok(expedition) => expedition.inserted_id.as_object_id().unwrap().to_hex(),
            Err(err) => return Err(err),
        };

        let new_user_doc = doc! {
            "$push": {
                "organized_expeditions": {
                    "exp_id": expedition_id,
                    "name": &new_expedition.name,
                    "start_date": &new_expedition.start_time,
                }
            }
            };

            let user_id = ObjectId::parse_str(user_id).unwrap();
            let user_filter = doc!{"_id":user_id};

            let updated_user_doc = self
            .user_col
            .update_one(user_filter, new_user_doc, None)
            .ok()
            .expect("Error updating user");
        
        Ok(updated_user_doc)
    }


    pub fn add_expedition_to_user(
        &self,
        user_id: &String,
        expedition_id: &String
    ) -> Result<UpdateResult, Error> {
        
        let expedition_detail = match self.get_expedition(expedition_id) {
            Ok(expedition) => expedition,
            Err(err) => return Err(err),
        };

        if expedition_detail.participants.len() == expedition_detail.max_no_participants as usize {
            panic!("Max number of participants reached!");
        }

        let user_detail = match self.get_user(user_id) {
            Ok(user) => user,
            Err(err) => return Err(err),
        };

        let new_user_doc = doc! {
            "$push": {
                "my_expeditions": {
                    "exp_id": expedition_id,
                    "name": &expedition_detail.name,
                    "start_date": &expedition_detail.start_time,
                    "reserved": false,
                    "paid": false
                }
            }
        };

        let new_exp_doc = doc! {
            "$push": {
                "participants":{
                    "user_id": user_id,
                    "firstname": user_detail.firstname,
                    "lastname": user_detail.lastname,
                    "paid": false
                }
            }
        };

        let expedition_id = ObjectId::parse_str(expedition_id).unwrap();
        let user_id = ObjectId::parse_str(user_id).unwrap();
        let user_filter = doc!{"_id":user_id};
        let expedition_filter = doc!{"_id":expedition_id};

        let updated_expedition_doc = self
        .expedition_col
        .update_one(expedition_filter, new_exp_doc, None)
        .ok()
        .expect("Error updating expedition");

        self
        .user_col
        .update_one(user_filter, new_user_doc, None)
        .ok()
        .expect("Error updating user");

        Ok(updated_expedition_doc)
    }

    
    pub fn mark_expedition_as_paid(
        &self,
        user_id: &String,
        expedition_id: &String
    ) -> Result<UpdateResult, Error> {
      
        let expedition_id = match ObjectId::parse_str(expedition_id) {
            Ok(id) => id,
            Err(err) => return Err(Error::from(err)),
        };

        let user_id = match ObjectId::parse_str(user_id) {
            Ok(id) => id,
            Err(err) => return Err(Error::from(err)),
        };

        let user_filter = doc! {
            "_id": user_id,
            "my_expeditions.exp_id": expedition_id,
        };

        let expedition_filter = doc! {
            "_id": expedition_id,
            "participants.user_id": user_id,
        };

        // Update documents to set "paid": true
        let update_user_doc = doc! {
            "$set": {
                "my_expeditions.$.paid": true,
            },
        };

        let update_expedition_doc = doc! {
            "$set": {
                "participants.$.paid": true,
            },
        };


        let updated_user_doc = self
        .user_col
        .update_one(user_filter, update_user_doc, None)
        .ok()
        .expect("Error updating user");

        self
        .expedition_col
        .update_one(expedition_filter, update_expedition_doc, None)
        .ok()
        .expect("Error updating expedition");


        Ok(updated_user_doc)
    }


    pub fn make_user_participant(
        &self,
        id: &String,
        firstname: &String,
        lastname: &String
    ) -> Result<UpdateResult, Error> {

        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "firstname":firstname,
                    "lastname":lastname,
                    "my_expeditions":[],
                },
            "$push":
            {
                "role":"Participant"
            },
        };
        let updated_doc = self
        .user_col
        .update_one(filter, new_doc, None)
        .ok()
        .expect("Error updating user");

        Ok(updated_doc)
    }


    pub fn make_user_organizer(
        &self,
        id: &String,
        company_name: &String,
        contact: &String
    ) -> Result<UpdateResult, Error> {

        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
            {
                "company_name":company_name,
                "contact":contact,
                "organized_expeditions":[],
            },
            "$push":
            {
                "role":"Organizer"
            },
        };
        let updated_doc = self
        .user_col
        .update_one(filter, new_doc, None)
        .ok()
        .expect("Error updating user");
        
        Ok(updated_doc)
    }


    pub fn update_expedition(
        &self,
        id: &String,
        updated_expedition: Expedition
    ) -> Result<UpdateResult, Error> {

        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "name": updated_expedition.name,
                    "stops": updated_expedition.stops,
                    "max_no_participants": updated_expedition.max_no_participants,
                    "guide": {
                        "firstaname":updated_expedition.guide.firstname,
                        "lastname":updated_expedition.guide.lastname,
                        "age":updated_expedition.guide.age,
                        "experience":updated_expedition.guide.experience,},
                    // "organizer": {
                    //     "org_id":updated_expedition.organizer.unwrap().org_id,
                    //     "name":updated_expedition.organizer.unwrap().name,
                    // },
                    "start_time": updated_expedition.start_time,
                    "end_time": updated_expedition.end_time,
                    "home_station": updated_expedition.home_station,
                    "price": updated_expedition.price,
                },
        };
        let updated_doc = self
            .expedition_col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating expedition");
        
        Ok(updated_doc)
    }


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
            id: None, //mongoDB will create unique id
            login: new_user.login,
            password: new_user.password,
            role: new_user.role,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            company_name: new_user.company_name,
            my_expeditions: new_user.my_expeditions,
            organized_expeditions: new_user.organized_expeditions,
            contact: new_user.contact,
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


    pub fn find_user(&self, login: &String) -> Result<User, Error> {
        let stored_user = self
            .user_col
            .find_one(doc! { "login": &login }, None)
            .ok()
            .expect("Error finding user");
        Ok(stored_user.unwrap())
    }

    pub fn get_contacts(&self, stop: &String) -> Result<Vec<ContactOrganizator>,Error>{
        let contact_pipeline = vec![
        doc! {
            "$match": {
              "stops": stop
            }
          },
          doc! {
            "$lookup": {
              "from": "users",
              "let": { "orgId": { "$toObjectId": "$organizer.org_id" } },
              "pipeline": [
                {
                  "$match": {
                    "$expr": { "$eq": ["$_id", "$$orgId"] }
                  }
                }
              ],
              "as": "organizer_details"
            }
        },
          doc! {
            "$unwind": "$organizer_details"
          },
          doc! {
            "$project": {
              "_id": 0,
              "expedition_name": "$name",
              "organizer_name": "$organizer_details.login",
              "contact": "$organizer_details.contact"
            }
          }
];
let cursors = self
            .expedition_col
            .aggregate(contact_pipeline, None)
            .ok()
            .expect("Error getting list of contacts");
        let results: Result<Vec<ContactOrganizator>, Error> = cursors.map(|doc| {

            let expedition_name = doc.clone().unwrap().get_str("expedition_name").unwrap_or("").to_string();
            let organizer_name = doc.clone().unwrap().get_str("organizer_name").unwrap_or("").to_string();
            let contact = doc.clone().unwrap().get_str("contact").unwrap_or("").to_string();

            Ok(ContactOrganizator {
                expedition_name,
                organizer_name,
                contact,
            })
        }).collect();

        results
    }
}
