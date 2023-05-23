use async_std::{sync::Mutex, task};
use rand::{thread_rng, Rng};
use rust::running_stat::RunningStat;
use std::{env, sync::Arc};

#[async_std::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100_000;
    let num_tasks = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let max_temp = 100.0;
    let min_temp = 0.0;
    let mut rng = thread_rng();
    let stat = Arc::new(Mutex::new(RunningStat::new()));

    println!("Starting {} tasks", num_tasks);

    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        let temp = rng.gen_range(min_temp..=max_temp);
        let stat = stat.clone();
        tasks.push(task::spawn(async move {
            stat.lock().await.push(temp);
        }));
    }
    for task in tasks {
        task.await;
    }

    println!("All tasks complete");

    println!("Mean: {:.1}", stat.lock().await.mean());
    println!("Count: {}", stat.lock().await.data_value_count());
}
