mod config;
mod directory;

use crate::config::Config;
use crate::directory::print_tree;
use std::io;

fn main() -> io::Result<()> {
    let config = Config::new()?;
    print_tree(&config, &config.path, 0, &[])
}
