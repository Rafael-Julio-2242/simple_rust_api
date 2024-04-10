use actix_web::{web, HttpResponse, Responder, get, post, delete, patch};

use crate::controllers::notes_controller;
use crate::dtos::new_note_dto::NewNoteDTO;

#[get("")]
async fn get_all_notes() -> impl Responder {


    let notes_result = notes_controller::get_all_notes();

    if notes_result.is_err() {
        return HttpResponse::InternalServerError().body("Error getting notes");
    }

    let body = serde_json::to_string(&notes_result.unwrap()).unwrap();

    HttpResponse::Ok().body(body)

}


#[get("/{user_id}")]
async fn get_user_notes(path: web::Path<String>) -> impl Responder {

    let user_id = path.into_inner().parse::<i64>();

    if user_id.is_err() {
        return HttpResponse::BadRequest().body("Invalid user id");
    }

    let notes_result = notes_controller::get_user_notes(user_id.unwrap());

    if notes_result.is_err() {
        return HttpResponse::InternalServerError().body("Error getting notes");
    }

    let body = serde_json::to_string(&notes_result.unwrap()).unwrap();

    HttpResponse::Ok().body(body)
}

#[post("/new_note")]
async fn new_note(new_note: web::Json<NewNoteDTO>) -> impl Responder {

    let new_note_dto = new_note.into_inner();

    let r = notes_controller::new_note(new_note_dto.title, new_note_dto.description, new_note_dto.user_id);    

    if r.is_err() {
        return HttpResponse::InternalServerError().body("Error creating note");
    }

    HttpResponse::Ok().body("Note created successfully")

}

#[delete("/{id}")]
async fn delete_note(path: web::Path<String>) -> impl Responder {

    let note_id = path.into_inner().parse::<i64>();

    if note_id.is_err() {
        return HttpResponse::BadRequest().body("Invalid note id");
    }

    let r = notes_controller::delete_note(note_id.unwrap());

    if r.is_err() {
        return HttpResponse::InternalServerError().body(format!("Error deleting note: {}", r.unwrap_err()));
    }

    HttpResponse::Ok().body("Note deleted successfully")
}

pub fn get_notes_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notes")
            .service(get_all_notes)
            .service(get_user_notes)
            .service(new_note)
            .service(delete_note)
    );
}

