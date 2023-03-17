use actix_web::{web, App, HttpServer};
use common::web::handler_of;
use pulldepswithoverride::fs::foo_an_sfl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let arc_f = handler_of(foo_an_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
