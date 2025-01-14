use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Store dimensions
const STORE_LENGTH: usize = 4;
const STORE_HEIGHT: usize = 4;
const STORE_WIDTH: usize = 4;

// Single position in store
#[derive(Serialize, Deserialize, Clone)]
struct Position {
    item: Option<String>,
}

// Store structure (Warehouse)
#[derive(Serialize, Deserialize)]
struct Warehouse {
    positions: Vec<Vec<Vec<Position>>>,
}

impl Warehouse {
    // Initialize the warehouse with empty positions
    fn new() -> Self {
        let positions = vec![
            vec![
                vec![Position { item: None }; STORE_WIDTH]; // width
                STORE_HEIGHT
            ]; // height
            STORE_LENGTH
        ]; // length

        Warehouse { positions }
    }

    // Find all free positions
    fn get_free_positions(&self) -> Vec<(usize, usize, usize)> {
        let mut free_positions = Vec::new();
        for (x, layer) in self.positions.iter().enumerate() {
            for (y, row) in layer.iter().enumerate() {
                for (z, position) in row.iter().enumerate() {
                    if position.item.is_none() {
                        free_positions.push((x, y, z));
                    }
                }
            }
        }
        free_positions
    }
}

// AppState with thread safety
struct AppState {
    warehouse: Mutex<Warehouse>,
}

// Endpoint: server status
#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    let warehouse = Warehouse::new();
    let app_state = web::Data::new(AppState {
        warehouse: Mutex::new(warehouse),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share state
            .service(status) // Service for "/status"
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
