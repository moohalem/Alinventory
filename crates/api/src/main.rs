use actix_web::{web, App, HttpServer};
use clap::Parser;
use inventory_core::database::create_pool;
use inventory_core::inventory::Inventory;

// Declare the new modules
mod config;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Parse configuration from command-line arguments
    let args = config::Args::parse();

    // 2. Initialize database pool
    let pool = create_pool().expect("Failed to create database pool.");
    let inventory = web::Data::new(Inventory::new(pool));

    println!("Server running at http://0.0.0.0:{}", args.port);

    // 3. Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(inventory.clone())
            // Configure routes using the function from the handlers module
            .configure(handlers::configure_routes)
    })
    .bind(("0.0.0.0", args.port))?
    .run()
    .await
}
