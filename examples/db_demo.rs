use axum_best::conf::AppConf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 从配置文件加载配置
    let cfg = AppConf::from_path("etc/config.toml")?;

    println!("MySQL Configuration:");
    println!("  DSN: {}", cfg.mysql.dsn);
    println!("  Max Connections: {}", cfg.mysql.max_connections);
    println!("  Slow Level: {}", cfg.mysql.slow_level);
    println!("  Life Seconds: {}", cfg.mysql.lifetime_sec);
    println!("  Idle Seconds: {}", cfg.mysql.idle_sec);
    println!("  Timeout Level: {}", cfg.mysql.timeout_level);
    println!("  Slow Threshold (ms): {}", cfg.mysql.slow_threshold_mills);

    // 测试数据库连接
    println!("\nTesting database connection...");
    match cfg.mysql.init_conn().await {
        Ok(pool) => {
            println!("✅ Database connection successful!");

            // 测试查询
            match sqlx::query("SELECT 1").execute(&pool).await {
                Ok(_) => println!("✅ Database query test successful!"),
                Err(e) => println!("❌ Database query test failed: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Database connection failed: {}", e);
            println!("Please check your MySQL configuration in etc/config.toml");
        }
    }

    Ok(())
}
