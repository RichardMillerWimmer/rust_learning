use actix_web::{get, patch, post, web::Json, App, HttpResponse, HttpServer, Responder};
mod models;
use crate::models::pizza::BuyPizzaRequest;
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas Available!")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("pizza_name required"),
    }
}

#[patch("/updatepizza/{id}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizza Updated!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server!");
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
