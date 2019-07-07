use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::future;
use futures::future::Future;

fn main() {
    HttpServer::new(|| App::new().route("/", web::to_async(handler)))
        .bind("127.0.0.1:8000")
        .expect("Cannot bind to port 8000")
        .run()
        .expect("Unable to run server");
}

fn handler(req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    has_client_header(&req)
        .and_then(|client| {
            operation_that_returns_future(client)
                .map_err(|_| ErrorInternalServerError("operation failed"))
        })
        .map(|result| HttpResponse::Ok().body(result))
}

fn has_client_header(req: &HttpRequest) -> impl Future<Item = String, Error = Error> {
    if let Some(Ok(client)) = req.headers().get("x-client-id").map(|h| h.to_str()) {
        future::ok(client.to_owned())
    } else {
        future::failed(ErrorBadRequest("invalid x-client-id header"))
    }
}

fn operation_that_returns_future(client: String) -> impl Future<Item = String, Error = ()> {
    future::ok(client)
}
