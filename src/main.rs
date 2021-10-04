use std::fs;
use std::env;
use actix_cors::Cors;
use actix_web::{post, App, HttpRequest, HttpResponse, HttpServer, Responder};

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
async fn echo(req_body: String, req: HttpRequest) -> impl Responder {
    let connection_info = req.connection_info();
    let host = connection_info.host().to_string();
    let mut filename = get_epoch_ms().to_string();
    let extension = ".html".to_owned();
    let mut path = "./output/".to_string();
    path = [path, host, "/".to_string()].join("").to_string();
    // Ensure target dir is present
    fs::create_dir_all(&path).unwrap();

    filename = [path, filename, extension].join("").to_string();
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    println!("Writing to {}", filename);
    let mut file = File::create(filename).expect("error");
    file.write_all(req_body.as_bytes()).expect("Failed to write file");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure output dir is present
    fs::create_dir_all("./output")?;
    println!("Listening on 8080");
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(echo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await.ok();

    Ok(())
}
