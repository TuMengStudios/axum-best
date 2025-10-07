use std::fmt::Debug;
use std::fmt::Display;

use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use smart_default::SmartDefault;
use sqlx::FromRow;

/// User information entity representing a user in the system
#[derive(FromRow, Debug, SmartDefault, Deserialize, Serialize)]
pub struct UserInfo {
    /// Unique identifier for the user
    pub id: i64,
    /// Display name of the user
    pub nick_name: String,
    /// URL or path to user's profile picture
    pub avatar: String,
    /// User's personal signature or bio
    pub signature: String,
    /// User's age
    pub age: u8,
    /// User's phone number
    pub phone: String,
    /// WeChat Open ID for authentication
    pub wx_open_id: String,
    /// Salt used for password hashing
    pub salt: String,
    /// Hashed password
    pub password: String,
    /// Timestamp when the user was created (Unix timestamp)
    pub created_at: i64,
    /// Timestamp when the user was last updated (Unix timestamp)
    pub updated_at: i64,
    /// Timestamp when the user was deleted (Unix timestamp, 0 if not deleted)
    pub deleted_at: i64,
    // .... other fields
}

impl UserInfo {
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.nick_name = name;
        self
    }

    pub fn set_age(&mut self, age: u8) -> &mut Self {
        self.age = age;
        self
    }

    pub fn set_avatar(&mut self, avatar: String) -> &mut Self {
        self.avatar = avatar;
        self
    }

    pub fn set_signature(&mut self, signature: String) -> &mut Self {
        self.signature = signature;
        self
    }

    pub fn set_phone(&mut self, phone: String) -> &mut Self {
        self.phone = phone;
        self
    }

    pub fn set_salt(&mut self, salt: String) -> &mut Self {
        self.salt = salt;
        self
    }

    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }

    pub fn set_created_at(&mut self, created_at: i64) -> &mut Self {
        self.created_at = created_at;
        self
    }

    pub fn set_updated_at(&mut self, updated_at: i64) -> &mut Self {
        self.updated_at = updated_at;
        self
    }

    pub fn set_deleted_at(&mut self, deleted_at: i64) -> &mut Self {
        self.deleted_at = deleted_at;
        self
    }

    pub fn set_wx_open_id(&mut self, wx_open_id: String) -> &mut Self {
        self.wx_open_id = wx_open_id;
        self
    }

    /// 生成一个随机的 UserInfo 实例
    ///
    /// # 示例
    /// ```
    /// use axum_best::models::user::UserInfo;
    ///
    /// let random_user = UserInfo::random();
    /// println!("随机用户: {:?}", random_user);
    /// ```
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let timestamp = chrono::Utc::now().timestamp();

        // 随机昵称列表
        let nick_names = vec![
            "张三",
            "李四",
            "王五",
            "赵六",
            "钱七",
            "孙八",
            "周九",
            "吴十",
            "小明",
            "小红",
            "小刚",
            "小丽",
            "小强",
            "小美",
            "小华",
            "小芳",
            "程序员",
            "设计师",
            "产品经理",
            "测试工程师",
            "运维工程师",
        ];

        // 随机签名列表
        let signatures = vec![
            "热爱编程的程序员",
            "喜欢探索新技术",
            "享受创造的过程",
            "追求代码的优雅",
            "热爱开源社区",
            "技术改变世界",
            "代码即艺术",
            "持续学习，不断进步",
            "简单就是美",
            "细节决定成败",
        ];

        // 随机头像URL列表
        let avatars = vec![
            "https://example.com/avatar1.jpg",
            "https://example.com/avatar2.jpg",
            "https://example.com/avatar3.jpg",
            "https://example.com/avatar4.jpg",
            "https://example.com/avatar5.jpg",
            "https://example.com/avatar6.jpg",
            "https://example.com/avatar7.jpg",
            "https://example.com/avatar8.jpg",
        ];

        // 随机盐值
        let salt: String = (0..16)
            .map(|_| {
                let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                let idx = rng.random_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();

        // 随机密码哈希
        let password: String = (0..32)
            .map(|_| {
                let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                let idx = rng.random_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();

        // 随机微信OpenID
        let wx_open_id: String = (0..28)
            .map(|_| {
                let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                let idx = rng.random_range(0..chars.len());
                chars.chars().nth(idx).unwrap()
            })
            .collect();

        // 随机手机号
        let phone = format!(
            "1{}{}{}{}{}{}{}{}{}{}",
            rng.random_range(3..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
            rng.random_range(0..=9),
        );

        UserInfo {
            id: rng.random_range(1000..100000),
            nick_name: nick_names[rng.random_range(0..nick_names.len())].to_string(),
            avatar: avatars[rng.random_range(0..avatars.len())].to_string(),
            signature: signatures[rng.random_range(0..signatures.len())].to_string(),
            age: rng.random_range(18..60),
            phone,
            wx_open_id,
            salt,
            password,
            created_at: timestamp - rng.random_range(0..31536000), // 一年内的随机时间
            updated_at: timestamp,
            deleted_at: 0,
        }
    }
}

#[derive(Debug)]
pub enum P {
    I32(i32),
    I64(i64),
    Bool(bool),
    Str(String),
}

impl Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::I64(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::Bool(arg0) => f.write_fmt(format_args!("{}", arg0)),
            Self::Str(arg0) => f.write_fmt(format_args!("{}", arg0)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::UserInfo;

    #[test]
    fn test_chain_setters() {
        let mut user = UserInfo {
            id: 1,
            nick_name: "".to_string(),
            avatar: "".to_string(),
            signature: "".to_string(),
            age: 0,
            phone: "".to_string(),
            wx_open_id: "".to_string(),
            salt: "".to_string(),
            password: "".to_string(),
            created_at: 0,
            updated_at: 0,
            deleted_at: 0,
        };

        // 测试链式调用
        user.set_name("张三".to_string())
            .set_age(25)
            .set_avatar("avatar.jpg".to_string())
            .set_signature("热爱编程".to_string())
            .set_phone("13800138000".to_string())
            .set_salt("random_salt".to_string())
            .set_password("hashed_password".to_string())
            .set_created_at(1696560000)
            .set_updated_at(1696560000)
            .set_deleted_at(0);

        // 验证设置的值
        assert_eq!(user.nick_name, "张三");
        assert_eq!(user.age, 25);
        assert_eq!(user.avatar, "avatar.jpg");
        assert_eq!(user.signature, "热爱编程");
        assert_eq!(user.phone, "13800138000");
        assert_eq!(user.salt, "random_salt");
        assert_eq!(user.password, "hashed_password");
        assert_eq!(user.created_at, 1696560000);
        assert_eq!(user.updated_at, 1696560000);
        assert_eq!(user.deleted_at, 0);
    }

    #[test]
    fn test_partial_chain_setters() {
        let mut user = UserInfo {
            id: 2,
            nick_name: "".to_string(),
            avatar: "".to_string(),
            signature: "".to_string(),
            age: 0,
            phone: "".to_string(),
            wx_open_id: "".to_string(),
            salt: "".to_string(),
            password: "".to_string(),
            created_at: 0,
            updated_at: 0,
            deleted_at: 0,
        };

        // 测试部分链式调用
        user.set_name("李四".to_string())
            .set_age(30)
            .set_phone("13900139000".to_string());

        // 验证设置的值
        assert_eq!(user.nick_name, "李四");
        assert_eq!(user.age, 30);
        assert_eq!(user.phone, "13900139000");
        // 其他字段保持默认值
        assert_eq!(user.avatar, "");
        assert_eq!(user.signature, "");
    }

    #[test]
    fn test_random_user_generation() {
        // 生成多个随机用户，确保每次生成的数据都不同
        let user1 = UserInfo::random();
        let user2 = UserInfo::random();
        let user3 = UserInfo::random();

        // 验证基本字段不为空
        assert!(!user1.nick_name.is_empty());
        assert!(!user1.avatar.is_empty());
        assert!(!user1.signature.is_empty());
        assert!(!user1.phone.is_empty());
        assert!(!user1.wx_open_id.is_empty());
        assert!(!user1.salt.is_empty());
        assert!(!user1.password.is_empty());

        // 验证年龄范围
        assert!(user1.age >= 18 && user1.age <= 60);
        assert!(user2.age >= 18 && user2.age <= 60);
        assert!(user3.age >= 18 && user3.age <= 60);

        // 验证ID范围
        assert!(user1.id >= 1000 && user1.id < 100000);
        assert!(user2.id >= 1000 && user2.id < 100000);
        assert!(user3.id >= 1000 && user3.id < 100000);

        // 验证时间戳
        assert!(user1.created_at > 0);
        assert!(user1.updated_at > 0);
        assert_eq!(user1.deleted_at, 0);

        // 验证手机号格式
        assert!(user1.phone.starts_with('1'));
        assert_eq!(user1.phone.len(), 11);

        // 验证盐值和密码长度
        assert_eq!(user1.salt.len(), 16);
        assert_eq!(user1.password.len(), 32);
        assert_eq!(user1.wx_open_id.len(), 28);

        // 验证生成的用户数据不完全相同（随机性）
        assert_ne!(user1.nick_name, user2.nick_name);
        assert_ne!(user1.avatar, user2.avatar);
        assert_ne!(user1.signature, user2.signature);
        assert_ne!(user1.phone, user2.phone);
        assert_ne!(user1.wx_open_id, user2.wx_open_id);
        assert_ne!(user1.salt, user2.salt);
        assert_ne!(user1.password, user2.password);
    }
}

#[allow(unused)]
#[tokio::test]
async fn data() {
    use std::collections::HashMap;

    use sqlx::MySql;

    let mut map = HashMap::new();
    map.insert("s".to_string(), P::Bool(true));
    map.insert("key".to_string(), P::Str("hello".to_string()));
    map.insert("num".to_string(), P::I64(233));
    #[allow(non_snake_case)]
    let mut sqlBuilder: sqlx::QueryBuilder<MySql> = sqlx::QueryBuilder::new("");
    for ele in map.iter() {
        sqlBuilder.push(format!("{} = ?", ele.0));
        sqlBuilder.push_bind(format!("{}", ele.1));
    }
    println!("data {:#?}", map);
}
