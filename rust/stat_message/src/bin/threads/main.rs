use rand::{thread_rng, Rng};
use stat_message::running_stat::RunningStat;
use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100;
    let num_threads = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let max = 100.0;
    let min = 0.0;
    let mut stat = RunningStat::new();
    let (tx, rx) = mpsc::channel();

    println!("Starting {} threads", num_threads);

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            let num = rng.gen_range(min..=max);
            tx.send(num).unwrap();
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

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(num) = rx.recv() {
        stat.push(num);
    }

    println!("All messages received");

    println!("Mean: {:.1}", stat.mean());
    println!("Count: {}", stat.data_value_count());
}
