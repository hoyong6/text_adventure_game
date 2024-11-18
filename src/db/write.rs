use rusqlite::{params, Connection, Result};
use std::path::Path;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(serde::Deserialize)]
pub struct UpdateVillager {
    name: String,
    count: i32,
}


pub fn update_villager_count(db_path: &Path, name: &str, count: i32) -> Result<()> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "UPDATE villagers SET count = ?1 WHERE name = ?2",
        params![count, name],
    )?;
    Ok(())
}

// Update 需要的参数是name和count
pub async fn update_villager(
    db_path: web::Data<std::sync::Arc<std::path::PathBuf>>,
    item: web::Json<UpdateVillager>,
) -> impl Responder {
    let db_path = db_path.as_ref();

    match update_villager_count(db_path, &item.name, item.count) {
        Ok(_) => HttpResponse::Ok().body(r#"{"code": "200"}"#),
        Err(e) => {
            eprintln!("Failed to update villager count: {}", e);
            HttpResponse::InternalServerError().body("Failed to update villager count.")
        }
    }
}