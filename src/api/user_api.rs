use crate::{models::{expedition_model::Expedition, user_model::{User, UserForm, USER_PASSWORD_SALT}}, repository::mongodb_repo::MongoRepo};
use argon2::Config;
use rocket::form::Form;
use mongodb::results::{InsertOneResult,UpdateResult};
use rocket::{http::Status, request::FlashMessage, response::{Flash, Redirect}, serde::json::{self, Json}, State};
use rocket_dyn_templates::Template;
use rocket_dyn_templates::serde::json::json;

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {

    let user: User = new_user.into_inner(); //change from Json<User> to User
    let user_detail = db.create_user(user);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<path>")]
pub fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);
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

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let user = db.get_all_users();
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/<path>", data = "<new_expedition>")]
pub fn add_expedition_to_organizator(
    db:&State<MongoRepo>,
    path:String,
    new_expedition:Json<Expedition>) ->
    Result<Json<UpdateResult>, Status> {
        let user_id = path;
        let expedition: Expedition = new_expedition.into_inner();
        let result = db.add_expedition_to_organizator(&user_id, expedition);
        match result {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(Status::InternalServerError),
        }
    }

 #[get("/signup")]
pub fn signup_page(flash: Option<FlashMessage<'_>>) -> Template {
        Template::render(
            "signup", 
            json!({
                "flash": flash.map(FlashMessage::into_inner)
            })
        )
}



#[post("/createaccount", data = "<user_form>")]
pub async fn create_account(db: &State<MongoRepo>, user_form: Form<UserForm>) -> Flash<Redirect> {
    let user = user_form.into_inner();

    let hash_config = Config::default();
    let hash = match argon2::hash_encoded(user.password.as_bytes(), USER_PASSWORD_SALT, &hash_config) {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/signup"), "Issue creating account");
        }
    };

    let active_user = User {
        id: None,
        login: user.login,
        password: hash,
        role: [].to_vec(),
        firstname: None,
        lastname: None,
        company_name: None,
        my_expeditions: None,
        organized_expeditions: None,
        contact: None,
    };

    match db.create_user(active_user) {
        Ok(_) => Flash::success(Redirect::to("/login"), "Account created successfully!"),
        Err(e) => {
            eprintln!("Error creating user: {:?}", e); // Debugging line
            Flash::error(Redirect::to("/signup"), "Issue creating account")
        }
    }
}



#[get("/login")]
pub fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
    Template::render(
        "login", 
        json!({
            "flash": flash.map(FlashMessage::into_inner)
        })
    )
}