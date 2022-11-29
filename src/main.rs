use std::{
    fs,
    io::{BufWriter, Write},
    thread,
    time::{Duration, SystemTime},
};

use directories::ProjectDirs;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rdev::{listen, Event, EventType};

static WHEEL_LOG: Lazy<Mutex<Vec<(SystemTime, i8)>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() -> std::io::Result<()> {
    let proj_dirs = ProjectDirs::from("com", "reiyw", "mwheel-log").unwrap();
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir)?;
    let out_path = data_dir.join(format!(
        "{}.txt",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(30 * 60));
        {
            let file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&out_path)
                .unwrap();
            let mut buffer = BufWriter::new(file);

            let mut lock = WHEEL_LOG.lock();
            for row in lock.iter() {
                buffer
                    .write_all(
                        format!(
                            "{}\t{}\n",
                            row.0
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_millis(),
                            row.1
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }

            lock.clear();
        }
    });

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }

    Ok(())
}

fn callback(event: Event) {
    if let EventType::Wheel { delta_y, .. } = event.event_type {
        let mut lock = WHEEL_LOG.lock();
        lock.push((SystemTime::now(), delta_y as i8));
    }
}
