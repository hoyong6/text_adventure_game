use rusqlite::{Connection, Result};
use std::path::Path;

#[derive(Debug)]
pub struct VillagerType {
    pub job_name: String,
    pub count: i32,
}

pub fn read_villagers_from_db(db_path: &Path, user_id: &str) -> Result<Vec<VillagerType>> {
    let conn = Connection::open(db_path)?;
    // 获取指定用户的所有职业数量
    let mut villager_types = Vec::new();
    let row = conn.query_row(
        "SELECT woodcutter, meat_worker, miner, hunter, blacksmith, steelworker, tanner, trapper 
         FROM villagers 
         WHERE user_id = ?1",
        [user_id],
        |row| {
            Ok((
                row.get::<_, i32>(0)?, // woodcutter
                row.get::<_, i32>(1)?, // meat_worker
                row.get::<_, i32>(2)?, // miner
                row.get::<_, i32>(3)?, // hunter
                row.get::<_, i32>(4)?, // blacksmith
                row.get::<_, i32>(5)?, // steelworker
                row.get::<_, i32>(6)?, // tanner
                row.get::<_, i32>(7)?, // trapper
            ))
        }
    )?;
    // 将查询结果转换为 VillagerType 结构
    let job_counts = [
        ("伐木工", row.0),
        ("熏肉师", row.1),
        ("矿工", row.2),
        ("猎人", row.3),
        ("铁匠", row.4),
        ("炼钢工人", row.5),
        ("皮匠", row.6),
        ("陷阱师", row.7),
    ];
    for (name, count) in job_counts.iter() {
        if *count > 0 {  // 可选：只添加数量大于0的职业
            villager_types.push(VillagerType {
                job_name: name.to_string(),
                count: *count,
            });
        }
    }
    Ok(villager_types)
}