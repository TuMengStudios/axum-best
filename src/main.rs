use anyhow::Context as _;
use axum_best::app::AppContext;
use axum_best::conf;
use clap::Parser;
use human_panic::setup_panic;

#[derive(clap::Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    next_line_help = true
)]
struct Args {
    /// config path
    #[arg(long, default_value = "etc/config.toml", short)]
    conf: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    setup_panic!();

    let args = Args::parse();
    let cfg = conf::AppConf::from_path(&args.conf).context("parser conf file error")?;

    let app_context = AppContext::new(cfg)
        .await
        .context("build app context error")?;

    //
    app_context.start().await.context("start server error")?;
    Ok(())
}
