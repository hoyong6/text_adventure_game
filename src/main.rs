mod db;
mod game;
mod handlers;
mod websocket;
// 使用 mod db; 来声明 db 模块。
// 使用 use db::write::update_villager; 来引入 write.rs 中的函数，以便在 main.rs 中使用。
// 确保在 write.rs 中定义的函数是 pub 以便可以被外部访问。

use actix_web::{web, App, HttpServer};
use handlers::villager_handler::get_villager_types;
// use game::start_game;
use websocket::websocket_route;
use std::path::PathBuf;
use std::sync::Arc;
use db::write::update_villager;
use db::create_user::create_user_with_related_records;

use std::env;
use rusqlite::Connection;
// use log::LevelFilter;

fn init_logging() {
    // env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_LOG", "info");
    env_logger::init();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = PathBuf::from("/Users/heyong/Documents/Game");
    
    let conn = Connection::open(&db_path)?;
    create_user_with_related_records(&conn, "new_username")?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 指定数据库文件的路径
    let db_path = PathBuf::from("/Users/heyong/Documents/Game");

    // 启动游戏（注意：这可能会阻塞主线程，考虑将其移到单独的线程）
    // start_game(db_path.clone());

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }

    // 将 db_path 包装在 Arc 中以便在多个线程间共享
    let db_path = Arc::new(db_path);

    init_logging(); // 初始化日志

    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_path.clone()))
            .route("/ws/", web::get().to(websocket_route))
            .route("/villager_types", web::post().to(get_villager_types))
            .route("/update_villager", web::post().to(update_villager))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
