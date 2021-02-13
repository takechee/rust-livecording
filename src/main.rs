use std::fs;
use std::io;
use std::path::Path;
use chrono::{DateTime, TimeZone, Local};

fn main() {
    let result = read_dir("./");

    if result.is_ok() {
        println!("OK");
    }
}

fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if !metadata.is_dir() {
            let modified = metadata.modified();
            if modified.is_err() {
                continue;
            }

            let duration = modified.unwrap().duration_since(
                std::time::SystemTime::UNIX_EPOCH
            );

            if duration.is_err() {
                continue;
            }

            let datetime: DateTime<Local> = Local.timestamp(duration.unwrap().as_secs() as i64, 0);
            println!("{}, {}", datetime, entry.path().display().to_string());
        }
    }
    Ok(String::from("OK"))
}