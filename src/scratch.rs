use chrono::{DateTime, Local};
use std::env;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let metadata = fs::metadata(path).expect("Failed to read file metadata");
    let modified = metadata
        .modified()
        .expect("Failed to get modification time");

    let duration = modified
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp_us: u64 =
        duration.as_secs() * 1_000_000 + (duration.subsec_nanos() / 1_000) as u64;

    let datetime: DateTime<Local> = DateTime::from(modified);

    println!(
        "{} ({})",
        datetime.format("%Y-%m-%d %H:%M:%S%.f"),
        timestamp_us
    );
}

fn datetime_from_timestamp_us(timestamp_us: u64) -> DateTime<Local> {
    let secs = timestamp_us / 1_000_000;
    let nanos = (timestamp_us % 1_000_000) * 1_000;
    let system_time = UNIX_EPOCH + Duration::new(secs, nanos as u32);
    DateTime::<Local>::from(system_time)
}
