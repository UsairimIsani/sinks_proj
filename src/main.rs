use chrono::Utc;
use clap::{crate_version, Clap};
use clokwerk::{Scheduler, TimeUnits};

use msinks::prelude::*;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = "by sriram")]
pub struct Opts {
    /// Path to Config File
    #[clap(short, long, default_value = "config.yaml")]
    pub config: String,

    /// Demo Mode
    #[clap(long)]
    pub demo: bool,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    log::info!("Getting CLI Options");

    let cli = Opts::parse();

    log::info!("Parsing Settings from Config");

    let settings = Settings::from_file(&cli.config, cli.demo)?;

    let expires = settings.get_expires()?;

    let sink = settings.get_sink();

    let sink_type = sink.get("type").unwrap().as_str();

    let stores = Vec::new();

    let mut scheduler = Scheduler::with_tz(Utc);

    scheduler.every(1.hours()).run(move || {
        let expiry = expires;
        let current = Utc::now().date();
        if current >= expiry {
            std::process::exit(0);
        }
    });
    serve(SinkType::from(sink_type), sink.clone(), stores).await?;

    Ok(())
}
