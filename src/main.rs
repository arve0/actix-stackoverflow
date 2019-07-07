use actix_web::client::Client;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(Client::new())
            .route("/", web::to_async(handler))
    })
    .bind("127.0.0.1:8000")
    .expect("Cannot bind to port 8000")
    .run()
    .expect("Unable to run server");
}

fn handler(client: web::Data<Client>) -> impl Future<Item = HttpResponse, Error = Error> {
    client.get("https://gensho.ftp.acc.umu.se/debian-cd/current/amd64/iso-cd/debian-10.0.0-amd64-netinst.iso")
        .send()
        .map_err(Error::from)
        .and_then(|res| {
            HttpResponse::build(res.status()).streaming(res)
        })
}
