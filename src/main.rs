use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn main() {
    HttpServer::new(|| App::new().route("/", web::to(handler)))
        .bind("127.0.0.1:8000")
        .expect("Cannot bind to port 8000")
        .run()
        .expect("Unable to run server");
}

fn handler(req: HttpRequest) -> impl Responder {
    if let Some(content_type) = get_content_type(&req) {
        format!("Got content-type = '{}'", content_type)
    } else {
        "No content-type header.".to_owned()
    }
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("content-type")?.to_str().ok()
}
