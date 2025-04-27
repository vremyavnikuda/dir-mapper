use std::path::PathBuf;
use std::{env, io};

pub struct Config {
    pub(crate) path: PathBuf,
    pub(crate) max_depth: usize,
    pub(crate) show_hidden: bool,
    pub(crate) ignore_patterns: Vec<String>,
}

impl Config {
    pub(crate) fn new() -> io::Result<Self> {
        let mut args = env::args();
        let mut config = Config {
            path: PathBuf::from("."),
            max_depth: usize::MAX,
            show_hidden: false,
            ignore_patterns: Vec::new(),
        };

        args.next();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-a" => config.show_hidden = true,
                "-D" => {
                    if let Some(depth) = args.next() {
                        config.max_depth = depth.parse().unwrap_or(usize::MAX);
                    }
                }
                "-i" => {
                    if let Some(patterns) = args.next() {
                        config.ignore_patterns =
                            patterns.split(',').map(|s| s.to_string()).collect();
                    }
                }
                path => config.path = PathBuf::from(path),
            }
        }

        Ok(config)
    }
}
