use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer};

fn main() {
    HttpServer::new(move || App::new().route("/", web::to(index)))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .run()
        .unwrap();
}

fn index(_: Authorized) -> HttpResponse {
    HttpResponse::Ok().body("authorized")
}

struct Authorized;

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Result<Self, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if is_authorized(req) {
            Ok(Authorized)
        } else {
            Err(ErrorUnauthorized("not authorized"))
        }
    }
}

fn is_authorized(req: &HttpRequest) -> bool {
    if let Some(value) = req.headers().get("authorized") {
        // actual implementation that checks header here
        dbg!(value);
        true
    } else {
        false
    }
}
