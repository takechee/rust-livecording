use std::fs;
use std::io;
use std::path::Path;
use chrono::{DateTime, TimeZone, Local};
use std::result::Result::Err;
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<DateTime<Local>, Vec<String>> = HashMap::new();

    match read_dir(&mut map, "./") {
        Ok(_) => print(map),
        Err(e) => eprintln!("{}", e)
    }
}

fn print(map: HashMap<DateTime<Local>, Vec<String>>) {
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by_key(|a| a.0);

    for (key, values) in sorted {
        for value in values {
            println!("{} {}", key, value);
        }
    }
}

fn read_dir<P: AsRef<Path>>(
    map: &mut HashMap<DateTime<Local>, Vec<String>>,
    path: P,
) -> io::Result<String> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            if let Err(e) = read_dir(map, entry.path().display().to_string()) {
                eprintln!("{}", e);
            }
        } else {
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

            let datetime: DateTime<Local> = Local.timestamp(
                duration.unwrap().as_secs() as i64,
                0,
            );

            // KEYが存在すれば files として Vec<String> を受け取る
            if let Some(files) = map.get_mut(&datetime) {
                files.push(entry.path().display().to_string());
            } else {
                let filename = vec![entry.path().display().to_string()];
                map.insert(datetime, filename);
            }
        }
    }

    Ok(String::from("OK"))
}