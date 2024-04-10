use actix_web::{web, HttpResponse, Responder, get, post, delete, patch};

use crate::dtos::new_user_dto::NewUserDTO;
use crate::controllers::user_controller;
use crate::dtos::update_user_dto::UpdateUserDTO;


#[get("")]
async fn get_users() -> impl Responder {

    let users_result = user_controller::get_all_users();

    match users_result {
        Ok(users) => {
            let body = serde_json::to_string(&users).unwrap();

            HttpResponse::Ok().body(body)
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error getting users");
        }
    }

  

}

#[get("/{id}")]
async fn get_user(path: web::Path<String>) -> impl Responder {

    let id = path.into_inner().parse::<i64>();

    if id.is_err() {
        return HttpResponse::BadRequest().body("Invalid id");
    }

    let user_result = user_controller::get_user(id.unwrap());

    if user_result.is_err() {
        return HttpResponse::NotFound().body("User not found");
    }

    let body = serde_json::to_string(&user_result.unwrap()).unwrap();

    HttpResponse::Ok().body(body)

}

#[post("/new_user")]
async fn new_user(new_user: web::Json<NewUserDTO> ) -> impl Responder {
    let new_user = new_user.into_inner();

    if new_user.name.is_empty() || new_user.password.is_empty() {
        return HttpResponse::BadRequest().body("Invalid name or password");
    }

    let r = user_controller::new_user(new_user.name, new_user.password);

    if r.is_err() {
        return HttpResponse::InternalServerError().body("Error creating user");
    }

    HttpResponse::Ok().body("User created successfully")
}

#[delete("/{id}")]
async fn delete_user(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner().parse::<i64>();

    if id.is_err() {
        return HttpResponse::BadRequest().body("Invalid id");
    }

    let user_result = user_controller::delete_user(id.unwrap());

    if user_result.is_err() {
        return HttpResponse::NotFound().body("User not found");
    }

    HttpResponse::Ok().body("User deleted successfully")

}

#[patch("/update_user")]
async fn update_user(update_user: web::Json<UpdateUserDTO>) -> impl Responder {

    let update_user = update_user.into_inner();

    if update_user.name.is_none() && update_user.password.is_none() {
        return HttpResponse::BadRequest().body("Nothing to change...");
    }

    let r = user_controller::update_user(update_user.id, update_user.name, update_user.password);

    if r.is_err() {
        return HttpResponse::InternalServerError().body("Error updating user");
    }

    HttpResponse::Ok().body("User updated successfully")

}

pub fn get_users_route_config(cfg: &mut web::ServiceConfig) {
    
    cfg.service(
        web::scope("/users")
            .service(get_users)
            .service(get_user)
            .service(new_user)
            .service(delete_user)
            .service(update_user)
    );
}
