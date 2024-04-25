use actix_web::{get, patch, post, HttpResponse, Responder};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas Available!")
}

fn main() {
    println!("Hello, world!");
}
