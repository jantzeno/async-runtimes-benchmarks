use rand::{thread_rng, Rng};
use stat_mutex::running_stat::RunningStat;
use std::{env, sync::Arc};
use tokio::{sync::Mutex, task};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let default = 100_000;
    let num_tasks = match args.len() {
        2 => args[1].parse().unwrap_or(default),
        _ => default,
    };

    let max = 100.0;
    let min = 0.0;
    let mut rng = thread_rng();
    let stat = Arc::new(Mutex::new(RunningStat::new()));

    println!("Starting {} tasks", num_tasks);

    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        let num = rng.gen_range(min..=max);
        let stat = Arc::clone(&stat);
        tasks.push(task::spawn(async move {
            stat.lock().await.push(num);
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }

    println!("All tasks complete");

    println!("Mean: {:.1}", stat.lock().await.mean());
    println!("Count: {}", stat.lock().await.data_value_count());
}
