use sha2::{Digest, Sha512};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_pool = available_parallelism().unwrap().get();
    let default_tasks = 100_000;

    let num_tasks = match args.len() {
        2 => args[1].parse().unwrap_or(default_tasks),
        _ => default_tasks,
    };

    let hashes: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Create threadpool of {} threads", default_pool);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(default_pool)
        .build()
        .unwrap();

    for _ in 0..num_tasks / default_pool {
        let hashes: Arc<Mutex<Vec<String>>> = Arc::clone(&hashes);
        pool.broadcast(move |_| {
            // Generate and hash a UUID
            let uuid = Uuid::new_v4();
            let mut hasher = Sha512::new();
            hasher.update(uuid.as_bytes());
            let hash = hasher.finalize();

            // Store the hash
            let mut hashes = hashes.lock().unwrap();
            hashes.push(format!("{:x}", hash));
        });
    }

    println!("Broadcast complete");

    println!("Hash Count: {}", hashes.lock().unwrap().len());
}
