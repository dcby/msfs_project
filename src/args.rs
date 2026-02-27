use clap::Parser;
use msfs_static::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[repr(C)]
pub struct Args {
    #[arg(short, long)]
    pub author: String,

    #[arg(short, long, default_value_t = false)]
    pub force: bool,

    #[arg(short, long)]
    pub icao: String,

    #[arg(short, long)]
    pub name: String,

    #[arg(long)]
    pub name_slug: Option<String>,

    #[arg(short, long)]
    pub vendor: String,

    #[arg(long)]
    pub vendor_code: Option<String>,

    pub path: Option<String>,
}

impl From<&Args> for Config {
    fn from(value: &Args) -> Self {
        Self::new(
            &value.author,
            &value.icao,
            value.force,
            &value.name,
            value.name_slug.as_ref(),
            value.path.as_ref(),
            &value.vendor,
            value.vendor_code.as_ref(),
        )
    }
}
