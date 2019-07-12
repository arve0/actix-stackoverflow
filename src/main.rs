use actix_web::{web, App, Error, HttpServer, HttpRequest, HttpResponse, FromRequest};
use actix_web::dev::Payload;
use actix_web::http::header::{COOKIE, SET_COOKIE};
use actix_web::error::ErrorUnauthorized;

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/set", web::to(set_cookie))
            .route("/get", web::to(get_cookie))
    })
    .bind("127.0.0.1:8000")
    .expect("Cannot bind to port 8000")
    .run()
    .expect("Unable to run server");
}

fn set_cookie() -> HttpResponse {
    HttpResponse::Ok()
        .header(SET_COOKIE, Session::cookie("0123456789abcdef"))
        .body("cookie set")
}

fn get_cookie(session: Session) -> HttpResponse {
    HttpResponse::Ok()
        .header(SET_COOKIE, Session::cookie("new_session_value"))
        .body(format!("Got cookie {}", &session.0))
}

struct Session(String);

impl Session {
    const COOKIE_NAME: &'static str = "my-session";

    fn cookie(value: &str) -> String {
        String::from(Self::COOKIE_NAME) + "=" + value
    }
}

impl FromRequest for Session {
    type Error = Error;
    type Future = Result<Self, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        for header in req.headers().get_all(COOKIE) {
            // check if header is UTF-8
            if let Ok(value) = header.to_str() {
                // split into cookie values
                for c in value.split(';').map(|s| s.trim()) {
                    // split at '='
                    if let Some(pos) = c.find('=') {
                        // is session key?
                        if Self::COOKIE_NAME == &c[0..pos] {
                            return Ok(Session(String::from(&c[(pos + 1)..])));
                        }
                    }
                }
            }
        }
        Err(ErrorUnauthorized("Session cookie missing"))
    }
}
