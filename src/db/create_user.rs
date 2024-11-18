use rusqlite::{Connection, Result, Error};
use uuid::Uuid;

pub fn create_user_with_related_records(conn: &Connection, username: &str) -> Result<()> {
    // 首先检测用户名是否存在
    let mut stmt = conn.prepare("SELECT id FROM users WHERE username = ?")?;
    let count: i64 = stmt.query_row([username], |row| row.get(0))?;

    if count > 0 {
      return Err(Error::SqliteFailure(
        rusqlite::ffi::Error::new(4001),  // 使用自定义错误代码
        Some("Username already exists".to_string())
      ));
    }

    let user_id = Uuid::new_v4().to_string();

    conn.execute(
        "BEGIN TRANSACTION",
        [],
    )?;

    // 插入用户记录
    conn.execute(
        "INSERT INTO users (id, username) VALUES (?, ?)",
        [&user_id, username],
    )?;

    // 插入技能记录 夺命连环剑, 九阳神功, 小李飞刀, 千里眼
    conn.execute(
        "INSERT INTO skills (user_id, sword_combo, nine_yang, flying_dagger, eagle_eye) 
         VALUES (?, 0, 0, 0, 0)",
        [&user_id],
    )?;

    // 插入村民记录 伐木工, 熏肉师, 矿工, 猎人, 铁匠, 炼钢工人, 皮匠, 陷阱师
    conn.execute(
        "INSERT INTO villagers (user_id, woodcutter, meat_worker, miner, hunter, 
                                blacksmith, steelworker, tanner, trapper)
         VALUES (?, 0, 0, 0, 0, 0, 0, 0, 0)",
        [&user_id],
    )?;

    // 插入建筑记录 陷阱, 小推车, 小木屋, 猎人小屋, 交易栈, 制革坊, 工具房, 钢铁炉
    conn.execute(
        "INSERT INTO buildings (user_id, trap, cart, hut, lodge, 
                                trading_post, tannery, workshop, steelworks)
         VALUES (?, 0, 0, 0, 0, 0, 0, 0, 0)",
        [&user_id],
    )?;

    // 插入物品记录 外星合金 诱饵 子弹 饰品 布料 煤 熟肉 燃料电池 毛皮 铁 皮革 药物 鳞片 钢 利齿 火把 木头
    conn.execute("INSERT INTO inventory (user_id, alien_alloy, bait, bullet, amulet, cloth, coal, cured_meat, fuel_cell, fur, iron, leather, medicine, scales, steel, teeth, torch, wood) VALUES (?, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)", 
             [&user_id])?;

    // 插入武器记录 骨矛, 铁剑, 钢剑
    conn.execute(
        "INSERT INTO weapons (user_id, bone_spear, iron_sword, steel_sword)
         VALUES (?, 0, 0, 0)",
        [&user_id],
    )?;

    conn.execute("COMMIT", [])?;

    Ok(())
}
