task_count? = 10000
thread_count? = 1000

all_rust_stat_mutex: rust_stat_mutex_threads rust_stat_mutex_rayon rust_stat_mutex_tokio rust_stat_mutex_async_std

rust_stat_mutex_async_std:
	(cd rust && cargo run --release -p stat_mutex --bin async_std $(task_count))

rust_stat_mutex_rayon:
	(cd rust && cargo run --release -p stat_mutex --bin rayon $(task_count))

rust_stat_mutex_threads:
	(cd rust && cargo run --release -p stat_mutex --bin threads $(thread_count))

rust_stat_mutex_tokio:
	(cd rust && cargo run --release -p stat_mutex --bin tokio $(task_count))

all_rust_stat_message: rust_message_threads rust_message_tokio

rust_stat_message_threads:
	(cd rust && cargo run --release -p stat_message --bin threads $(thread_count))

rust_stat_message_tokio:
	(cd rust && cargo run --release -p stat_message --bin tokio $(task_count))

all_rust_hash_mutex: rust_hash_mutex_threads rust_hash_mutex_rayon rust_hash_mutex_tokio rust_hash_mutex_async_std

rust_hash_mutex_async_std:
	(cd rust && cargo run --release -p hash_mutex --bin async_std $(task_count))

rust_hash_mutex_rayon:
	(cd rust && cargo run --release -p hash_mutex --bin rayon $(task_count))

rust_hash_mutex_threads:
	(cd rust && cargo run --release -p hash_mutex --bin threads $(thread_count))

rust_hash_mutex_tokio:
	(cd rust && cargo run --release -p hash_mutex --bin tokio $(task_count))

all_rust_hash_message: rust_hash_message_threads rust_hash_message_tokio

rust_hash_message_threads:
	(cd rust && cargo run --release -p hash_message --bin threads $(thread_count))

rust_hash_message_tokio:
	(cd rust && cargo run --release -p hash_message --bin tokio $(task_count))