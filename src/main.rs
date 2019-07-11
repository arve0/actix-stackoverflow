use actix_web::{guard, web, App, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    username: String,
}

fn main() {
    HttpServer::new(|| {
        App::new().service(
web::resource("/")
    .route(
        web::post()
            .guard(guard::Header(
                "content-type",
                "application/x-www-form-urlencoded",
            ))
            .to(form_handler),
    )
    .route(
        web::post()
            .guard(guard::Header("content-type", "application/json"))
            .to(json_handler),
    ),
        )
    })
    .bind("127.0.0.1:8000")
    .expect("Cannot bind to port 8000")
    .run()
    .expect("Unable to run server");
}

fn form_handler(user: web::Form<User>) -> String {
    handler(user.into_inner())
}

fn json_handler(user: web::Json<User>) -> String {
    handler(user.into_inner())
}

fn handler(user: User) -> String {
    format!("Got username: {}", user.username)
}
