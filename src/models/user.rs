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
    /// Account status, see [`UserInfo::STATUS_NORMAL`] / [`UserInfo::STATUS_DISABLED`]
    pub status: i8,
    // .... other fields
}

impl UserInfo {
    /// Account is active and allowed to log in
    pub const STATUS_NORMAL: i8 = 0;
    /// Account has been disabled (e.g. banned by an admin)
    pub const STATUS_DISABLED: i8 = 1;

    /// Creates a local user record for a first-time external login.
    pub fn new_external() -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: 0,
            nick_name: "新用户".to_string(),
            avatar: String::new(),
            signature: String::new(),
            age: 0,
            phone: String::new(),
            salt: String::new(),
            password: String::new(),
            created_at: now,
            updated_at: now,
            deleted_at: 0,
            status: Self::STATUS_NORMAL,
        }
    }

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

    pub fn set_status(&mut self, status: i8) -> &mut Self {
        self.status = status;
        self
    }

    /// Generates a random UserInfo instance
    ///
    /// # Example
    /// ```
    /// use axum_best::models::user::UserInfo;
    ///
    /// let random_user = UserInfo::random();
    /// println!("random user: {:?}", random_user);
    /// ```
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let timestamp = chrono::Utc::now().timestamp();

        // Extended random nickname list for more variety
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
            "开发者",
            "架构师",
            "前端工程师",
            "后端工程师",
            "全栈工程师",
            "技术总监",
            "项目经理",
            "数据分析师",
            "算法工程师",
            "安全工程师",
            "网络工程师",
            "系统管理员",
            "数据库管理员",
            "移动开发",
            "游戏开发",
            "人工智能",
            "机器学习",
            "深度学习",
            "大数据",
            "云计算",
            "物联网",
            "区块链",
            "元宇宙",
            "虚拟现实",
            "增强现实",
            "数字孪生",
            "边缘计算",
            "量子计算",
            "生物信息",
            "机器人",
        ];

        // Extended random signature list for more variety
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
            "追求极致性能",
            "关注用户体验",
            "热爱分享知识",
            "乐于帮助他人",
            "享受团队协作",
            "追求技术创新",
            "关注行业动态",
            "热爱生活和工作",
            "平衡工作与生活",
            "享受编程乐趣",
            "探索未知领域",
            "挑战技术难题",
            "追求代码质量",
            "注重工程实践",
            "热爱开源精神",
            "关注前沿技术",
            "享受学习过程",
            "追求个人成长",
            "热爱技术社区",
            "乐于交流分享",
            "关注产品价值",
            "追求商业成功",
            "热爱创造价值",
            "享受解决问题",
            "关注用户需求",
            "追求卓越品质",
        ];

        // Generate a more unique nickname by appending a random suffix
        let base_nick_name = nick_names[rng.random_range(0..nick_names.len())];
        let digits = b"0123456789";
        let nick_name_suffix: String = (0..4)
            .map(|_| digits[rng.random_range(0..digits.len())] as char)
            .collect();
        let nick_name = format!("{}{}", base_nick_name, nick_name_suffix);

        // Generate a more unique signature with a random prefix or suffix
        let base_signature = signatures[rng.random_range(0..signatures.len())];
        let signature_variants = [
            base_signature.to_string(),
            format!("{}的{}", nick_name, base_signature),
            format!("{} - {}", base_signature, chrono::Utc::now().format("%Y")),
            format!("{} | {}", base_signature, rng.random_range(1000..9999)),
        ];
        let signature = signature_variants[rng.random_range(0..signature_variants.len())].clone();

        // Generate a fully random avatar URL
        let avatar_chars = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let avatar_id: String = (0..16)
            .map(|_| avatar_chars[rng.random_range(0..avatar_chars.len())] as char)
            .collect();

        // Random salt
        let alphanumeric = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let salt: String = (0..16)
            .map(|_| alphanumeric[rng.random_range(0..alphanumeric.len())] as char)
            .collect();

        // Random password hash
        let password: String = (0..32)
            .map(|_| alphanumeric[rng.random_range(0..alphanumeric.len())] as char)
            .collect();

        // Random phone number
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
            nick_name,
            avatar: format!("https://example.com/avatar_{}.jpg", avatar_id),
            signature,
            age: rng.random_range(18..60),
            phone,
            salt,
            password,
            created_at: timestamp - rng.random_range(0..31536000), // random time within the past year
            updated_at: timestamp,
            deleted_at: 0,
            status: Self::STATUS_NORMAL,
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
            salt: "".to_string(),
            password: "".to_string(),
            created_at: 0,
            updated_at: 0,
            deleted_at: 0,
            status: 0,
        };

        // Test chained setters
        user.set_name("张三".to_string())
            .set_age(25)
            .set_avatar("avatar.jpg".to_string())
            .set_signature("热爱编程".to_string())
            .set_phone("13800138000".to_string())
            .set_salt("random_salt".to_string())
            .set_password("hashed_password".to_string())
            .set_created_at(1696560000)
            .set_updated_at(1696560000)
            .set_deleted_at(0)
            .set_status(UserInfo::STATUS_NORMAL);

        // Verify the values that were set
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
        assert_eq!(user.status, UserInfo::STATUS_NORMAL);
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
            salt: "".to_string(),
            password: "".to_string(),
            created_at: 0,
            updated_at: 0,
            deleted_at: 0,
            status: 0,
        };

        // Test partial chained setters
        user.set_name("李四".to_string())
            .set_age(30)
            .set_phone("13900139000".to_string());

        // Verify the values that were set
        assert_eq!(user.nick_name, "李四");
        assert_eq!(user.age, 30);
        assert_eq!(user.phone, "13900139000");
        // Other fields keep their default values
        assert_eq!(user.avatar, "");
        assert_eq!(user.signature, "");
    }

    #[test]
    fn test_random_user_generation() {
        // Generate several random users and ensure each generation differs
        let user1 = UserInfo::random();
        let user2 = UserInfo::random();
        let user3 = UserInfo::random();

        // Verify basic fields are not empty
        assert!(!user1.nick_name.is_empty());
        assert!(!user1.avatar.is_empty());
        assert!(!user1.signature.is_empty());
        assert!(!user1.phone.is_empty());
        assert!(!user1.salt.is_empty());
        assert!(!user1.password.is_empty());

        // Verify the age range
        assert!(user1.age >= 18 && user1.age <= 60);
        assert!(user2.age >= 18 && user2.age <= 60);
        assert!(user3.age >= 18 && user3.age <= 60);

        // Verify the ID range
        assert!(user1.id >= 1000 && user1.id < 100000);
        assert!(user2.id >= 1000 && user2.id < 100000);
        assert!(user3.id >= 1000 && user3.id < 100000);

        // Verify timestamps
        assert!(user1.created_at > 0);
        assert!(user1.updated_at > 0);
        assert_eq!(user1.deleted_at, 0);

        // Verify the phone number format
        assert!(user1.phone.starts_with('1'));
        assert_eq!(user1.phone.len(), 11);

        // Verify salt and password lengths
        assert_eq!(user1.salt.len(), 16);
        assert_eq!(user1.password.len(), 32);

        // Verify the generated user data is not all identical (randomness)
        assert_ne!(user1.nick_name, user2.nick_name);
        assert_ne!(user1.avatar, user2.avatar);
        assert_ne!(user1.signature, user2.signature);
        assert_ne!(user1.phone, user2.phone);
        assert_ne!(user1.salt, user2.salt);
        assert_ne!(user1.password, user2.password);
    }

    #[test]
    fn test_new_external_user() {
        let user = UserInfo::new_external();

        assert_eq!(user.id, 0);
        assert_eq!(user.nick_name, "新用户");
        assert_eq!(user.deleted_at, 0);
        assert_eq!(user.status, UserInfo::STATUS_NORMAL);
        assert!(user.created_at > 0);
        assert_eq!(user.created_at, user.updated_at);
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
