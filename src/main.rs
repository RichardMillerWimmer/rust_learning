use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas Available!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server!");
    HttpServer::new(|| App::new().service(get_pizzas))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
