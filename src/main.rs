#[macro_use]
extern crate rocket;

mod models;
mod routes;


#[get("/")]
fn index() -> String {
    String::from("ROCKEEET")
}

#[rocket::main]
async fn main() {

    let figment = rocket::Config::figment()
        .merge(("port", 3000));




    let _ = rocket::custom(figment)
        .mount("/", routes![index])
        .launch().await;


}
