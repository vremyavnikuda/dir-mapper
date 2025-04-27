use crate::config::Config;
use std::path::Path;
use std::{env, fs, io};

pub fn should_skip(entry: &Path, config: &Config) -> bool {
    let name = entry
        .file_name()
        .unwrap_or(entry.as_os_str())
        .to_string_lossy();

    (!config.show_hidden && name.starts_with('.'))
        || config.ignore_patterns.iter().any(|p| {
            let p = p.to_lowercase();
            let name = name.to_lowercase();
            p == name || (p.starts_with("*.") && name.ends_with(&p[1..]))
        })
}

pub fn print_tree(
    config: &Config,
    path: &Path,
    depth: usize,
    is_last_entries: &[bool],
) -> io::Result<()> {
    if depth > config.max_depth {
        return Ok(());
    }

    if depth == 0 {
        let dir_name = env::current_dir()?
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| ".".to_string());
        println!("{}", dir_name);
    } else {
        for i in 0..depth - 1 {
            print!("{}", if is_last_entries[i] { "    " } else { "│   " });
        }
        print!(
            "{} ",
            if is_last_entries[depth - 1] {
                "└──"
            } else {
                "├──"
            }
        );
        println!("{}", path.file_name().unwrap().to_string_lossy());
    }

    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|e| !should_skip(e, config))
            .collect();

        entries.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            b_is_dir
                .cmp(&a_is_dir)
                .then(a.file_name().cmp(&b.file_name()))
        });

        for (i, entry) in entries.iter().enumerate() {
            let mut new_last = is_last_entries.to_vec();
            new_last.push(i == entries.len() - 1);
            print_tree(config, entry, depth + 1, &new_last)?;
        }
    }

    Ok(())
}
