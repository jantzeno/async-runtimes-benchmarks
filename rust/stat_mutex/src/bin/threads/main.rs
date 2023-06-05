use rand::{thread_rng, Rng};
use stat_mutex::running_stat::RunningStat;
use std::env;
use std::sync::{Arc, Mutex};
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
    let mut rng = thread_rng();
    let stat = Arc::new(Mutex::new(RunningStat::new()));

    println!("Starting {} threads", num_threads);

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let num = rng.gen_range(min..=max);
        let stat = Arc::clone(&stat);
        let handle = thread::spawn(move || stat.lock().unwrap().push(num));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads joined");

    println!("Mean: {:.1}", stat.lock().unwrap().mean());
    println!("Count: {}", stat.lock().unwrap().data_value_count());
}
