mod api;
mod models;
mod repository;

#[macro_use] extern crate rocket;
extern crate argon2;

use api::expedition_api::{
    add_expedition_to_user, create_expedition, delete_expedition,
    get_all_expeditions, get_contact_raport,get_contact_raport_form, get_expedition
};
use api::user_api::{
    create_account, create_user, delete_user, get_all_users, get_user, login_page, mark_expedition_as_paid, signup_page, verify_account
};
use api::other_apis::get_account;
use repository::mongodb_repo::MongoRepo;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket::fs::relative;
use std::collections::HashMap;


#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<i32, i32>::new())
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .attach(Template::fairing())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![index, signup_page, create_account, login_page, verify_account])
        .mount("/", routes![create_expedition])
        .mount("/", routes![get_expedition])
        // .mount("/", routes![update_expedition])
        .mount("/", routes![delete_expedition])
        .mount("/", routes![get_all_expeditions, get_contact_raport, get_contact_raport_form])
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
        // .mount("/", routes![add_expedition_to_organizator, add_expedition_to_user])
        .mount("/", routes![add_expedition_to_user,mark_expedition_as_paid])
        .mount("/", routes![get_account])
}
