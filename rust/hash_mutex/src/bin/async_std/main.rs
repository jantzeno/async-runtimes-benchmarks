use async_std::{sync::Mutex, task};
use sha2::{Digest, Sha512};
use std::{env, sync::Arc};
use uuid::Uuid;

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100_000;
    let num_tasks = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let hashes: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Starting {} tasks", num_tasks);

    let mut tasks = Vec::new();

    for _ in 0..num_tasks {
        let hashes = Arc::clone(&hashes);
        tasks.push(task::spawn(async move {
            // Generate and hash a UUID
            let uuid = Uuid::new_v4();
            let mut hasher = Sha512::new();
            hasher.update(uuid.as_bytes());
            let hash = hasher.finalize();

            // Store the hash
            let mut hashes = hashes.lock().await;
            hashes.push(format!("{:x}", hash));
        }));
    }

    for task in tasks {
        task.await;
    }

    println!("All tasks complete");

    println!("Hash Count: {}", hashes.lock().await.len());
}
