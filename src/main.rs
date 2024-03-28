#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Crabs can into space!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}