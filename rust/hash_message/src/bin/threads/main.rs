use sha2::{Digest, Sha512};
use std::env;
use std::sync::mpsc;
use std::thread;
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100;
    let num_threads = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let (tx, rx) = mpsc::channel();

    println!("Starting {} threads", num_threads);

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            // Generate and hash a UUID
            let uuid = Uuid::new_v4();
            let mut hasher = Sha512::new();
            hasher.update(uuid.as_bytes());
            let hash = hasher.finalize();

            tx.send(hash).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads joined");

    // Drop the last sender to stop `rx` waiting for message.
    // The program will not complete if we comment this out.
    drop(tx);

    let mut hashes = Vec::new();

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(hash) = rx.recv() {
        hashes.push(hash);
    }

    println!("All messages received");

    println!("Hash Count: {}", hashes.len());
}
