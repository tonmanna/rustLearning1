use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// fn show_request(request: &actix_web::HttpRequest,) -> Box<Future<Item = HttpResponse, Error = Error>> {
//     Box::new(request.body().map_err(|e| e.into()).map(move |f| {
//         actix_web::HttpResponse::Ok()
//             .content_type("text/plain")
//             .body(f)
//     }))
// }

// pub fn index(scope: actix_web::Scope<()>) -> actix_web::Scope<()> {
//     scope.handler("", |req: &actix_web::HttpRequest| {
//         show_request(req)
//     })
// }

fn index(info: web::Path<(String, u32)>) -> impl Responder {
    format!("Hello {}! id:{}", info.0, info.1)
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
    HttpServer::new(|| App::new().service(web::resource("/").to(index2)))
        .bind("0.0.0.0:88")?
        .run()
}
