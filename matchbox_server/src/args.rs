use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(
    name = "made_in_heaven",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(default_value = "0.0.0.0:3536", env)]
    pub host: SocketAddr,
    pub cert: Option<PathBuf>,
    pub key: Option<PathBuf>,
}
