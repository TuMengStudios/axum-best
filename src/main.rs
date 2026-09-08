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
    let cfg = conf::AppConf::from_path(&args.conf)
        .map_err(|err| anyhow::anyhow!("parser conf file error {:?}", err))?;

    let app_context = AppContext::new(cfg)
        .await
        .map_err(|err| anyhow::anyhow!("build app context error {}", err))?;

    //
    app_context
        .start()
        .await
        .map_err(|err| anyhow::anyhow!("start server error {}", err))?;
    Ok(())
}
