use sha2::{Digest, Sha512};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100;
    let num_threads = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let hashes: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Starting {} threads", num_threads);

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let hashes = Arc::clone(&hashes);
        let handle = thread::spawn(move || {
            // Generate and hash a UUID
            let uuid = Uuid::new_v4();
            let mut hasher = Sha512::new();
            hasher.update(uuid.as_bytes());
            let hash = hasher.finalize();

            // Store the hash
            let mut hashes = hashes.lock().unwrap();
            hashes.push(format!("{:x}", hash));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads joined");

    println!("Hash Count: {}", hashes.lock().unwrap().len());
}
