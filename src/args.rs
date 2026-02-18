use clap::Parser;
use msfs_static::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    // /// Path to manifest.json
    // #[arg(short, long, default_value = "manifest.json")]
    // pub file: String,
    #[arg(short, long)]
    pub author: String,

    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    #[arg(short, long)]
    pub icao: String,

    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub vendor: String,

    pub path: Option<String>,
}

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Self::new(
            &value.author,
            &value.icao,
            value.force,
            &value.name,
            &value.vendor,
        )
    }
}
