//! 测试 UserInfo::random() 方法的唯一性

use axum_best::models::user::UserInfo;

fn main() {
    println!("生成 10 个随机用户，测试昵称和签名的唯一性...\n");

    let mut users = Vec::new();
    for i in 0..10 {
        let user = UserInfo::random();
        users.push(user);
        println!("用户 {}: 昵称 = {}, 签名 = {}", i + 1, users[i].nick_name, users[i].signature);
    }

    // 检查昵称重复
    let mut nick_name_set = std::collections::HashSet::new();
    let mut duplicate_nick_names = Vec::new();

    for user in &users {
        if !nick_name_set.insert(&user.nick_name) {
            duplicate_nick_names.push(user.nick_name.clone());
        }
    }

    // 检查签名重复
    let mut signature_set = std::collections::HashSet::new();
    let mut duplicate_signatures = Vec::new();

    for user in &users {
        if !signature_set.insert(&user.signature) {
            duplicate_signatures.push(user.signature.clone());
        }
    }

    println!("\n=== 唯一性分析 ===");
    println!("总用户数: {}", users.len());
    println!("唯一昵称数: {}", nick_name_set.len());
    println!("唯一签名数: {}", signature_set.len());

    if duplicate_nick_names.is_empty() {
        println!("✅ 没有重复的昵称");
    } else {
        println!("❌ 重复的昵称: {:?}", duplicate_nick_names);
    }

    if duplicate_signatures.is_empty() {
        println!("✅ 没有重复的签名");
    } else {
        println!("❌ 重复的签名: {:?}", duplicate_signatures);
    }

    // 生成更多用户进行更全面的测试
    println!("\n=== 扩展测试 (生成 100 个用户) ===");
    let mut large_nick_name_set = std::collections::HashSet::new();
    let mut large_signature_set = std::collections::HashSet::new();

    for _ in 0..100 {
        let user = UserInfo::random();
        large_nick_name_set.insert(user.nick_name);
        large_signature_set.insert(user.signature);
    }

    println!("100 个用户中的唯一昵称数: {}", large_nick_name_set.len());
    println!("100 个用户中的唯一签名数: {}", large_signature_set.len());
    println!("昵称重复率: {:.2}%", (100.0 - (large_nick_name_set.len() as f64 / 100.0 * 100.0)));
    println!("签名重复率: {:.2}%", (100.0 - (large_signature_set.len() as f64 / 100.0 * 100.0)));
}
