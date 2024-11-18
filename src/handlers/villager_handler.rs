use actix_web::{web, HttpResponse, Responder};
use std::path::PathBuf;
use std::sync::Arc;
use log::info; // 导入 log crate
use crate::db::read_type::read_villager_types;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct VillagerRequest {
    user_id: String,
}

pub async fn get_villager_types(db_path: web::Data<Arc<PathBuf>>, req_body: web::Json<VillagerRequest>,) -> impl Responder {
    println!("db_path: {:?}", db_path.as_ref().as_path());

    match read_villager_types(db_path.as_ref().as_path(), &req_body.user_id) {
        Ok(villager_types) => {
            info!("villager_types: {:?}", villager_types);

            HttpResponse::Ok().json(villager_types)
        },
        Err(e) => {
            eprintln!("Error retrieving villager types: {}", e);
            HttpResponse::InternalServerError().body(format!("Error retrieving villager types: {}", e))
        }
    }
}
