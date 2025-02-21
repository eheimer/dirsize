use std::env;
use std::fs;
use std::path::Path;
use std::fs::Metadata;
use std::io::ErrorKind;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dirpath = if args.len() > 1 { &args[1] } else { "." };

    println!("Checking {}", dirpath);

    let entries = fs::read_dir(dirpath).expect("Failed to read directory");

    let mut items = Vec::new();
    let mut total_size = 0;

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if let Some(metadata) = get_metadata(&path) {
            if metadata.is_file() {
                let size = metadata.len();
                total_size += size;
                items.push((size, path.display().to_string(), false));
            } else if metadata.is_dir() {
                let size = get_dir_size(&path);
                total_size += size;
                items.push((size, path.display().to_string(), true));
            }
        }
    }

    items.sort_by_key(|k| k.0);

    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();

    for (size, path, is_dir) in items {
        if is_dir {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
        } else {
            stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
        }
        writeln!(&mut stdout, "{}\t{}", format_size(size), path).unwrap();
    }

    stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
    writeln!(&mut stdout, "Total: {}", format_size(total_size)).unwrap();
}

fn get_metadata(path: &Path) -> Option<Metadata> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                None
            } else {
                fs::metadata(path).ok()
            }
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                // Ignore "No such file or directory" errors, typically for symlinks
                None
            } else {
                eprintln!("Failed to read metadata for {}: {}", path.display(), e);
                None
            }
        }
    }
}

fn get_dir_size(path: &Path) -> u64 {
    let mut total_size = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let entry = entry.expect("Failed to read entry");
            if let Some(metadata) = get_metadata(&entry.path()) {
                if metadata.is_file() {
                    total_size += metadata.len();
                } else if metadata.is_dir() {
                    total_size += get_dir_size(&entry.path());
                }
            }
        }
    }

    total_size
}

fn format_size(size: u64) -> String {
    let mut size = size as f64;
    let units = ["B", "K", "M", "G"];
    let mut unit = "B";

    for &u in &units {
        if size < 1024.0 {
            unit = u;
            break;
        }
        size /= 1024.0;
    }

    format!("{}{}", size as u64, unit)
}