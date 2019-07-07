use actix_web::client::{Client, SendRequestError};
use actix_web::error::{ErrorBadGateway, ErrorInternalServerError};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;

fn main() {
    HttpServer::new(|| App::new().data(Client::new()).route("/", web::to(handler)))
        .bind("127.0.0.1:8000")
        .expect("Cannot bind to port 8000")
        .run()
        .expect("Unable to run server");
}

fn handler(client: web::Data<Client>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    Box::new(
        client
            .get("https://httpbin.org/get")
            .no_decompress()
            .send()
            .map_err(|err| match err {
                SendRequestError::Connect(error) => {
                    ErrorBadGateway(format!("Unable to connect to httpbin: {}", error))
                }
                error => ErrorInternalServerError(error),
            })
            .and_then(|response| {
                let mut result = HttpResponse::build(response.status());
                let headers = response
                    .headers()
                    .iter()
                    .filter(|(h, _)| *h != "connection" && *h != "content-length");
                for (header_name, header_value) in headers {
                    result.header(header_name.clone(), header_value.clone());
                }
                Ok(result.streaming(response))
            }),
    )
}
