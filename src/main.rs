mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;
extern crate argon2;

use api::expedition_api::{create_expedition,get_expedition,delete_expedition,get_all_expeditions,add_expedition_to_user};
use api::user_api::{create_user,get_user,get_all_users,delete_user,add_expedition_to_organizator,
    signup_page,create_account, login_page};
use repository::mongodb_repo::MongoRepo;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket::fs::relative;

#[get("/")]
fn index() -> &'static str {
    "Crabs can into space!"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index,signup_page,create_account,login_page])
        .mount("/", routes![create_expedition])
        .mount("/", routes![get_expedition])
        // .mount("/",routes![update_expedition])
        .mount("/",routes![delete_expedition])
        .mount("/", routes![get_all_expeditions])
        .mount("/",routes![create_user])
        .mount("/",routes![get_user])
        .mount("/",routes![delete_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![add_expedition_to_organizator,add_expedition_to_user])
}