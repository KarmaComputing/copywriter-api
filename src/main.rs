use actix_cors::Cors;
use actix_web::{post, App, HttpResponse, HttpServer, Responder};

use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let mut filename = get_epoch_ms().to_string();
    let extension = ".html".to_owned();
    filename = [filename, extension].join("").to_string();
    let mut file = File::create(filename).expect("error");
    file.write_all(req_body.as_bytes()).expect("Failed to write file");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 8080");
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await.ok();

    Ok(())
}
