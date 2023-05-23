use std::env;
use std::sync::{Arc, Mutex};
use std::thread::available_parallelism;

use rand::{thread_rng, Rng};
use rust::running_stat::RunningStat;

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_pool = available_parallelism().unwrap().get();
    let default_tasks = 100_000;

    let num_tasks = match args.len() {
        2 => args[1].parse().unwrap_or(default_tasks),
        _ => default_tasks,
    };

    let max_temp = 100.0;
    let min_temp = 0.0;
    let mut rng = thread_rng();
    let stat = Arc::new(Mutex::new(RunningStat::new()));

    println!("Create threadpool of {} threads", default_pool);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(default_pool)
        .build()
        .unwrap();

    for _ in 0..num_tasks / default_pool {
        let temp = rng.gen_range(min_temp..=max_temp);
        let stat = stat.clone();
        pool.broadcast(move |_| stat.lock().unwrap().push(temp));
    }

    println!("Broadcast complete");

    println!("Mean: {:.1}", stat.lock().unwrap().mean());
    println!("Count: {}", stat.lock().unwrap().data_value_count());
}
