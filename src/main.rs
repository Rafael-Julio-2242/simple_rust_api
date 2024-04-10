use actix_web::{App, HttpServer};

mod routes;
mod controllers;
mod models;
mod dtos;

// importando rotas importantes!
use routes::users_route::get_users_route_config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Web Server started on port 8000");
    HttpServer::new(|| {   
        App::new()
            .configure(get_users_route_config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await


}
