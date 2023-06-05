use sha2::{Digest, Sha512};
use std::env;
use tokio::{sync::mpsc, task};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100_000;
    let num_tasks = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    println!("Starting {} tasks", num_tasks);

    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        let tx = tx.clone();
        tasks.push(task::spawn(async move {
            // Generate and hash a UUID
            let uuid = Uuid::new_v4();
            let mut hasher = Sha512::new();
            hasher.update(uuid.as_bytes());
            let hash = hasher.finalize();

            tx.send(format!("{:x}", hash)).unwrap();
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }

    println!("All tasks complete");

    // Drop the last sender, closing the channel.
    drop(tx);

    let mut hashes = Vec::new();

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(hash) = rx.try_recv() {
        hashes.push(hash);
    }

    println!("Hash Count: {}", hashes.len());
}
