use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

struct AppState {
    items: Mutex<HashMap<u32, Item>>,
}

async fn create_item(item: web::Json<Item>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    items.insert(item.id, item.into_inner());
    HttpResponse::Created().json("Item created")
}

async fn get_item(item_id: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let items = state.items.lock().unwrap();
    match items.get(&item_id.into_inner()) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json("Item not found"),
    }
}

async fn update_item(item_id: web::Path<u32>, item: web::Json<Item>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(existing_item) = items.get_mut(&item_id.into_inner()) {
        existing_item.name = item.name.clone();
        existing_item.description = item.description.clone();
        HttpResponse::Ok().json("Item updated")
    } else {
        HttpResponse::NotFound().json("Item not found")
    }
}

async fn delete_item(item_id: web::Path<u32>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if items.remove(&item_id.into_inner()).is_some() {
        HttpResponse::Ok().json("Item deleted")
    } else {
        HttpResponse::NotFound().json("Item not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        items: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::get().to(get_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
