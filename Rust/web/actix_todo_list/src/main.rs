use actix_web::{get,post,web,App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey!")
}

#[get("/index_page")]
async fn index_page(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Index Page {app_name}")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        // .service(
        //     web::scope("/app")
        //     .route("index", web::get().to(index))
        // )
        .service(index_page)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}