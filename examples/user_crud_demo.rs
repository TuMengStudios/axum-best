//! User CRUD operations demo
//!
//! This example shows how to use the UserRepo repository trait (contract defined in repos/user.rs,
//! MySQL implementation provided in data/user.rs)

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use axum_best::models::user::UserInfo;
use axum_best::repos::user::UserUpdate;

/// Returns the current timestamp
fn current_timestamp() -> Result<i64, Box<dyn std::error::Error>> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 用户 CRUD 操作演示 ===\n");

    // Note: in real usage you need to establish a database connection and construct the repository first:
    // let repo = MySqlUserRepo::new(pool); // Arc<dyn UserRepo>

    // 1. Create a user
    println!("1. 创建用户:");
    let new_user = UserInfo {
        id: 0,
        nick_name: "测试用户".to_string(),
        avatar: "avatar.jpg".to_string(),
        signature: "这是一个测试用户".to_string(),
        age: 25,
        phone: "13800138000".to_string(),
        salt: "random_salt".to_string(),
        password: "hashed_password".to_string(),
        created_at: current_timestamp()?,
        updated_at: current_timestamp()?,
        deleted_at: 0,
        status: UserInfo::STATUS_NORMAL,
    };

    println!("   创建前用户ID: {}", new_user.id);
    // In real usage: repo.create(&mut new_user).await?;
    println!("   创建后用户ID: {}", new_user.id);
    println!("   用户昵称: {}", new_user.nick_name);
    println!("   用户手机: {}\n", new_user.phone);

    // 2. Get a user by ID
    println!("2. 根据ID获取用户:");
    // In real usage: let user = repo.get_by_id(new_user.id).await?;
    println!("   可以获取用户ID为 {} 的用户信息\n", new_user.id);

    // 3. Get a user by phone number
    println!("3. 根据手机号获取用户:");
    // In real usage: let user = repo.get_by_phone("13800138000").await?;
    println!("   可以获取手机号为 {} 的用户信息\n", new_user.phone);

    // 4. Update user information
    println!("4. 更新用户信息:");
    let updated_user = UserInfo {
        id: new_user.id,
        nick_name: "更新后的用户".to_string(),
        avatar: new_user.avatar.clone(),
        signature: "这是更新后的签名".to_string(),
        age: new_user.age,
        phone: new_user.phone.clone(),
        salt: new_user.salt.clone(),
        password: new_user.password.clone(),
        created_at: new_user.created_at,
        updated_at: current_timestamp()?,
        deleted_at: new_user.deleted_at,
        status: new_user.status,
    };

    // In real usage: repo.update(&updated_user).await?;
    println!("   更新用户昵称为: {}", updated_user.nick_name);
    println!("   更新用户签名为: {}\n", updated_user.signature);

    // 5. Partially update user information
    println!("5. 部分更新用户信息:");
    let updates = vec![
        UserUpdate::NickName("部分更新昵称".to_string()),
        UserUpdate::Signature("部分更新签名".to_string()),
    ];
    // In real usage: repo.update_partial(new_user.id, &updates).await?;
    println!("   部分更新字段: {:?}\n", updates);

    // 6. List users
    println!("6. 获取用户列表:");
    // In real usage: let users = repo.list(1, 10).await?;
    println!("   可以获取第1页，每页10条的用户列表\n");

    // 7. Search users
    println!("7. 搜索用户:");
    // In real usage: let users = repo.search_by_nickname("测试", 1, 10).await?;
    println!("   可以搜索昵称包含'测试'的用户\n");

    // 8. Count users
    println!("8. 获取用户总数:");
    // In real usage: let count = repo.count().await?;
    println!("   可以获取未删除用户的总数\n");

    // 9. Soft delete a user
    println!("9. 软删除用户:");
    // In real usage: repo.delete(new_user.id, current_timestamp()?).await?;
    println!("   软删除用户ID为 {} 的用户\n", new_user.id);

    // 10. Hard delete a user
    println!("10. 硬删除用户:");
    // In real usage: repo.hard_delete(new_user.id).await?;
    println!("   硬删除用户ID为 {} 的用户\n", new_user.id);

    println!("=== CRUD 操作演示完成 ===");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_timestamp() {
        let timestamp = current_timestamp().expect("system time must be after UNIX_EPOCH");
        assert!(timestamp > 0);
    }
}
