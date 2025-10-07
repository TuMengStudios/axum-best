use sqlx::MySqlPool;
use sqlx::QueryBuilder;

use crate::core::rest::AppError;
use crate::data::mysql::covert_error;
use crate::models::user::UserInfo;

/// 创建用户
pub async fn create(conn: &MySqlPool, user: &mut UserInfo) -> Result<(), AppError> {
    user.id = sqlx::query_as!(UserInfo,
        r#"INSERT INTO user_info (nick_name, avatar, signature, age, phone, wx_open_id, salt, password, created_at, updated_at, deleted_at) 
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        user.nick_name,
        user.avatar,
        user.signature,
        user.age,
        user.phone,
        user.wx_open_id,
        user.salt,
        user.password,
        user.created_at,
        user.updated_at,
        user.deleted_at
    )
    .execute(conn)
    .await.map_err(|err|{
        covert_error(err)
    })?.last_insert_id() as i64;

    Ok(())
}

/// 更新用户信息
pub async fn update(conn: &MySqlPool, user: &UserInfo) -> Result<(), AppError> {
    sqlx::query!(
        r#"UPDATE user_info SET 
           nick_name = ?, avatar = ?, signature = ?, age = ?, phone = ?, 
           wx_open_id = ?, salt = ?, password = ?, updated_at = ?
           WHERE id = ?"#,
        user.nick_name,
        user.avatar,
        user.signature,
        user.age,
        user.phone,
        user.wx_open_id,
        user.salt,
        user.password,
        user.updated_at,
        user.id
    )
    .execute(conn)
    .await
    .map_err(|err| covert_error(err))?;

    Ok(())
}

/// 根据ID获取用户
pub async fn get_by_id(conn: &MySqlPool, id: i64) -> Result<UserInfo, AppError> {
    let user = sqlx::query_as!(UserInfo, r#"SELECT * FROM user_info WHERE id = ?"#, id)
        .fetch_one(conn)
        .await
        .map_err(|err| covert_error(err))?;
    Ok(user)
}

/// 根据手机号获取用户
pub async fn get_by_phone(conn: &MySqlPool, phone: &str) -> Result<UserInfo, AppError> {
    let user = sqlx::query_as!(UserInfo, r#"SELECT * FROM user_info WHERE phone = ?"#, phone)
        .fetch_one(conn)
        .await
        .map_err(|err| covert_error(err))?;
    Ok(user)
}

/// 根据微信Open ID获取用户
pub async fn get_by_wx_open_id(conn: &MySqlPool, wx_open_id: &str) -> Result<UserInfo, AppError> {
    let user =
        sqlx::query_as!(UserInfo, r#"SELECT * FROM user_info WHERE wx_open_id = ?"#, wx_open_id)
            .fetch_one(conn)
            .await
            .map_err(|err| covert_error(err))?;
    Ok(user)
}

/// 软删除用户（设置deleted_at时间戳）
pub async fn delete(conn: &MySqlPool, id: i64, deleted_at: i64) -> Result<(), AppError> {
    sqlx::query!(r#"UPDATE user_info SET deleted_at = ? WHERE id = ?"#, deleted_at, id)
        .execute(conn)
        .await
        .map_err(|err| covert_error(err))?;

    Ok(())
}

/// 硬删除用户（从数据库中完全删除）
pub async fn hard_delete(conn: &MySqlPool, id: i64) -> Result<(), AppError> {
    sqlx::query!(r#"DELETE FROM user_info WHERE id = ?"#, id)
        .execute(conn)
        .await
        .map_err(|err| covert_error(err))?;

    Ok(())
}

/// 获取用户列表（分页查询）
pub async fn list(conn: &MySqlPool, page: u32, page_size: u32) -> Result<Vec<UserInfo>, AppError> {
    let offset = (page - 1) * page_size;
    let users = sqlx::query_as!(
        UserInfo,
        r#"SELECT * FROM user_info WHERE deleted_at = 0 ORDER BY id DESC LIMIT ? OFFSET ?"#,
        page_size as i64,
        offset as i64
    )
    .fetch_all(conn)
    .await
    .map_err(|err| covert_error(err))?;

    Ok(users)
}

/// 获取用户总数
pub async fn count(conn: &MySqlPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM user_info WHERE deleted_at = 0"#)
        .fetch_one(conn)
        .await
        .map_err(|err| covert_error(err))?;

    Ok(count)
}

/// 根据昵称搜索用户
pub async fn search_by_nickname(
    conn: &MySqlPool,
    nickname: &str,
    page: u32,
    page_size: u32,
) -> Result<Vec<UserInfo>, AppError> {
    let offset = (page - 1) * page_size;
    let search_pattern = format!("%{}%", nickname);

    let users = sqlx::query_as!(
        UserInfo,
        r#"SELECT * FROM user_info 
           WHERE nick_name LIKE ? AND deleted_at = 0 
           ORDER BY id DESC LIMIT ? OFFSET ?"#,
        search_pattern,
        page_size as i64,
        offset as i64
    )
    .fetch_all(conn)
    .await
    .map_err(|err| covert_error(err))?;

    Ok(users)
}

/// 更新用户部分信息（使用QueryBuilder动态构建更新语句）
pub async fn update_partial(
    conn: &MySqlPool,
    id: i64,
    updates: &[(&str, &str)],
) -> Result<(), sqlx::Error> {
    if updates.is_empty() {
        return Ok(());
    }

    let mut query_builder = QueryBuilder::new("UPDATE user_info SET ");

    for (i, (field, value)) in updates.iter().enumerate() {
        if i > 0 {
            query_builder.push(", ");
        }
        query_builder.push(field);
        query_builder.push(" = ");
        query_builder.push_bind(value);
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);

    query_builder.build().execute(conn).await?;

    Ok(())
}
