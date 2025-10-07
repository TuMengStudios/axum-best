//! 用户 CRUD 操作演示
//!
//! 这个示例展示了如何使用 repos/user.rs 中实现的完整 CRUD 方法

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axum_best::models::user::UserInfo;

/// 获取当前时间戳
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 用户 CRUD 操作演示 ===\n");

    // 注意：在实际使用中，你需要先建立数据库连接
    // 这里只是展示方法的使用方式

    // 1. 创建用户
    println!("1. 创建用户:");
    let new_user = UserInfo {
        id: 0,
        nick_name: "测试用户".to_string(),
        avatar: "avatar.jpg".to_string(),
        signature: "这是一个测试用户".to_string(),
        age: 25,
        phone: "13800138000".to_string(),
        wx_open_id: "wx_openid_123456789".to_string(),
        salt: "random_salt".to_string(),
        password: "hashed_password".to_string(),
        created_at: current_timestamp(),
        updated_at: current_timestamp(),
        deleted_at: 0,
    };

    println!("   创建前用户ID: {}", new_user.id);
    // 在实际使用中调用: user::create(&pool, &mut new_user).await?;
    println!("   创建后用户ID: {}", new_user.id);
    println!("   用户昵称: {}", new_user.nick_name);
    println!("   用户手机: {}\n", new_user.phone);

    // 2. 根据ID获取用户
    println!("2. 根据ID获取用户:");
    // 在实际使用中调用: let user = user::get_by_id(&pool, new_user.id).await?;
    println!("   可以获取用户ID为 {} 的用户信息\n", new_user.id);

    // 3. 根据手机号获取用户
    println!("3. 根据手机号获取用户:");
    // 在实际使用中调用: let user = user::get_by_phone(&pool, "13800138000").await?;
    println!("   可以获取手机号为 {} 的用户信息\n", new_user.phone);

    // 4. 更新用户信息
    println!("4. 更新用户信息:");
    let updated_user = UserInfo {
        id: new_user.id,
        nick_name: "更新后的用户".to_string(),
        avatar: new_user.avatar.clone(),
        signature: "这是更新后的签名".to_string(),
        age: new_user.age,
        phone: new_user.phone.clone(),
        wx_open_id: new_user.wx_open_id.clone(),
        salt: new_user.salt.clone(),
        password: new_user.password.clone(),
        created_at: new_user.created_at,
        updated_at: current_timestamp(),
        deleted_at: new_user.deleted_at,
    };

    // 在实际使用中调用: user::update(&pool, &updated_user).await?;
    println!("   更新用户昵称为: {}", updated_user.nick_name);
    println!("   更新用户签名为: {}\n", updated_user.signature);

    // 5. 部分更新用户信息
    println!("5. 部分更新用户信息:");
    let updates = vec![("nick_name", "部分更新昵称"), ("signature", "部分更新签名")];
    // 在实际使用中调用: user::update_partial(&pool, new_user.id, &updates).await?;
    println!("   部分更新字段: {:?}\n", updates);

    // 6. 获取用户列表
    println!("6. 获取用户列表:");
    // 在实际使用中调用: let users = user::list(&pool, 1, 10).await?;
    println!("   可以获取第1页，每页10条的用户列表\n");

    // 7. 搜索用户
    println!("7. 搜索用户:");
    // 在实际使用中调用: let users = user::search_by_nickname(&pool, "测试", 1, 10).await?;
    println!("   可以搜索昵称包含'测试'的用户\n");

    // 8. 获取用户总数
    println!("8. 获取用户总数:");
    // 在实际使用中调用: let count = user::count(&pool).await?;
    println!("   可以获取未删除用户的总数\n");

    // 9. 软删除用户
    println!("9. 软删除用户:");
    // 在实际使用中调用: user::delete(&pool, new_user.id, current_timestamp()).await?;
    println!("   软删除用户ID为 {} 的用户\n", new_user.id);

    // 10. 硬删除用户
    println!("10. 硬删除用户:");
    // 在实际使用中调用: user::hard_delete(&pool, new_user.id).await?;
    println!("   硬删除用户ID为 {} 的用户\n", new_user.id);

    println!("=== CRUD 操作演示完成 ===");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let timestamp = current_timestamp();
        assert!(timestamp > 0);
    }
}
