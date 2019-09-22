use actix_web::web::{Data, ServiceConfig};
use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    let db = String::from("simplified example");

    HttpServer::new(move || App::new().configure(config_app(db.clone())))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .run()
        .unwrap();
}

fn config_app(db: String) -> Box<Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.data(db.clone())
            .service(web::resource("/notes").route(web::get().to(notes)));
    })
}

fn notes(db: Data<String>) -> HttpResponse {
    HttpResponse::Ok().body(["notes from ", &db].concat())
}
