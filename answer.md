actix-session [serializes session data to JSON](https://github.com/actix/actix-web/blob/f410f3330fb771e8d51b7448ea2b0d3981d95891/actix-session/src/lib.rs#L124), [signs the cookie](https://github.com/actix/actix-web/blob/f410f3330fb771e8d51b7448ea2b0d3981d95891/actix-session/src/cookie.rs#L110) and [sets the name of the cookie to actix-session](https://github.com/actix/actix-web/blob/master/actix-session/src/cookie.rs#L68).

To verify, run the [minimal cookie-session-example](https://github.com/actix/examples/tree/master/cookie-session) and do a request with curl:

```none
$ curl -v localhost:8080
> GET / HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.54.0
> Accept: */*
>
< HTTP/1.1 200 OK
< content-length: 8
< content-type: text/plain; charset=utf-8
< set-cookie: actix-session=ZTe%2Fb%2F085+VQcxL%2FQRKCnldUxzoc%2FNEOQe94PTBGUfc%3D%7B%22counter%22%3A%221%22%7D; HttpOnly; Path=/
< date: Thu, 11 Jul 2019 21:22:38 GMT
```

Decoding with [`decodeURIComponent`](https://developer.mozilla.org/de/docs/Web/JavaScript/Reference/Global_Objects/decodeURIComponent) gives:
```none
> decodeURIComponent("ZTe%2Fb%2F085+VQcxL%2FQRKCnldUxzoc%2FNEOQe94PTBGUfc%3D%7B%22counter%22%3A%221%22%7D")
'ZTe/b/085+VQcxL/QRKCnldUxzoc/NEOQe94PTBGUfc={"counter":"1"}'
```

As far as I know, `ZTe/b/085+VQcxL/QRKCnldUxzoc/NEOQe94PTBGUfc=` is the signature.

This is probably not excatly what your PHP script is doing, so you might want to use [`HttpRequest::headers`](https://docs.rs/actix-web/1.0.3/actix_web/struct.HttpRequest.html#method.headers) directly. For example, by creating your own `Session` type, then using that type in your handlers:


```rust
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
```

Result (irrelevant headers removed for brevity):
```none
$ curl -v localhost:8000/get
< HTTP/1.1 401 Unauthorized
Session cookie missing⏎

$ curl -v localhost:8000/set
< HTTP/1.1 200 OK
< set-cookie: my-session=0123456789abcdef
cookie set⏎

$ curl -v --cookie my-session=0123456789abcdef localhost:8000/get
> Cookie: my-session=0123456789abcdef
>
< HTTP/1.1 200 OK
< set-cookie: my-session=new_session_value
Got cookie 0123456789abcdef⏎
```

You may also observe the results in a browser, urls http://localhost:8000/set and http://localhost:8000/get.

This is quite simplistic, but gives you full control over the session cookies.

**NOTE:** The solution above does nothing to secure the cookies.
