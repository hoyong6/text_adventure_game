use crate::db::read::{self};
use std::path::PathBuf;
// self 指的是 crate::db::read 模块本身。通过引入 self，你可以直接使用该模块中的函数、结构体等，而不需要每次都写出完整的路径。
// 其实game 这里读了一把，然后接口调用的时候又读了一把
#[allow(dead_code)]
pub fn start_game(db_path: PathBuf, user_id: &str) {
    println!("db_path: {:?}", db_path);
    // 读取村民信息
    let villagers = match read::read_villagers_from_db(&db_path, &user_id) {
        
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read villagers from DB: {}", e);
            return;
        }
    };
    // 打印
    for villager in villagers {
        println!("{:?}", villager);
        println!("{:?}", villager);
    }
}