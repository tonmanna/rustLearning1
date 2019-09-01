use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use readcsv;
use readfs;
use std::process;

// fn index(info: web::Path<(String, u32)>) -> impl Responder {
//     format!("Hello {}! id:{}", info.0, info.1)
// }

//Month,Display URL domain,Impression share,Avg. position,Overlap rate,Position above rate,Top of page rate,Abs. Top of page rate,Outranking share
struct CSVData {
    month: f32,
    display: f32,
    impression: f32,
    avgpos: f32,
    overlap: f32,
    position: f32,
    top: f32,
    top2: f32,
    outrank: f32,
}

fn index2() -> impl Responder {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    HttpResponse::Ok()
        .content_type("application/json")
        .body(data)
}

fn main() -> std::io::Result<()> {
    match readfs::ReadFS("2.csv") {
        Ok(value) => {
            println!("CSV TEXT: {}", value);
            match readcsv::readCSV(value) {
                Ok(_) => println!("GOOD"),
                Err(e) => {
                    println!("CSV: {}", e);
                    //process::exit(1);
                }
            };
        }
        Err(e) => {
            println!("FILE OPEN: {}", e);
        }
    }

    HttpServer::new(|| App::new().service(web::resource("/").to(index2)))
        .bind("0.0.0.0:88")?
        .run()
}
