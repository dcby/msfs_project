mod args;

use args::Args;
use clap::Parser;
use msfs_static::{Config, create_msfs_project};
use std::process;

fn main() {
    let args = Args::parse();
    let cfg = Config::from(&args);

    create_msfs_project(&cfg).unwrap_or_else(|err| {
        eprintln!("Problem creating project: {err}");
        process::exit(1);
    });
}
