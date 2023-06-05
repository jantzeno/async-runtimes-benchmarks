use rand::{thread_rng, Rng};
use stat_message::running_stat::RunningStat;
use std::env;
use tokio::{sync::mpsc, task};

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
    let mut stat = RunningStat::new();
    let (tx, mut rx) = mpsc::unbounded_channel::<f64>();

    println!("Starting {} tasks", num_tasks);

    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        let tx = tx.clone();
        tasks.push(task::spawn(async move {
            let mut rng = thread_rng();
            let num = rng.gen_range(min..=max);
            tx.send(num).unwrap();
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }

    println!("All tasks complete");

    // Drop the last sender, closing the channel.
    drop(tx);

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(num) = rx.try_recv() {
        stat.push(num);
    }

    println!("Mean: {:.1}", stat.mean());
    println!("Count: {}", stat.data_value_count());
}
