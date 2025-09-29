use actix_web::{get, post, delete, web, App, HttpServer, HttpResponse, Responder};
use inventory_core::database::create_pool;
use inventory_core::inventory::Inventory;
use serde::Deserialize;
use rust_embed::Embed;

// Embed static files into the binary
#[derive(Embed)]
#[folder = "templates/"] // Corrected path: relative to crates/api/Cargo.toml
struct Templates;

#[derive(Embed)]
#[folder = "static/"] // Corrected path: relative to crates/api/Cargo.toml
struct StaticFiles;

// --- API Handlers ---

#[get("/ingredients")]
async fn get_ingredients(inventory: web::Data<Inventory>) -> impl Responder {
    match inventory.list_ingredients() {
        Ok(ingredients) => HttpResponse::Ok().json(ingredients),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct AddIngredientRequest {
    name: String,
    quantity: u32,
    unit: String,
}

#[post("/ingredients")]
async fn add_ingredient(inventory: web::Data<Inventory>, item: web::Json<AddIngredientRequest>) -> impl Responder {
    match inventory.add_ingredient(item.name.clone(), item.quantity, item.unit.clone()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
struct DeleteRequest {
    names: Vec<String>,
}

#[delete("/ingredients")]
async fn delete_ingredients(inventory: web::Data<Inventory>, req: web::Json<DeleteRequest>) -> impl Responder {
    for name in &req.names {
        if let Err(_) = inventory.delete_ingredient(name.clone()) {
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

// --- Embedded Static File Handlers ---

async fn serve_index() -> impl Responder {
    match Templates::get("index.html") {
        Some(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("index.html not found"),
    }
}

async fn serve_js() -> impl Responder {
    match StaticFiles::get("app.js") {
        Some(content) => HttpResponse::Ok()
            .content_type("application/javascript")
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("app.js not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_pool().expect("Failed to create database pool.");
    let inventory = web::Data::new(Inventory::new(pool));

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(inventory.clone())
            .service(
                web::scope("/api")
                    .service(get_ingredients)
                    .service(add_ingredient)
                    .service(delete_ingredients)
            )
            .route("/", web::get().to(serve_index))
            .route("/static/app.js", web::get().to(serve_js))
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
