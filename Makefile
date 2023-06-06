task_count = 100
thread_count = 100

rust_stat_mutex_async_std:
	(cd rust && cargo run --release -p stat_mutex --bin async_std $(task_count))

rust_stat_mutex_rayon:
	(cd rust && cargo run --release -p stat_mutex --bin rayon $(task_count))

rust_stat_mutex_threads:
	(cd rust && cargo run --release -p stat_mutex --bin threads $(thread_count))

rust_stat_mutex_tokio:
	(cd rust && cargo run --release -p stat_mutex --bin tokio $(task_count))

rust_stat_message_threads:
	(cd rust && cargo run --release -p stat_message --bin threads $(thread_count))

rust_stat_message_tokio:
	(cd rust && cargo run --release -p stat_message --bin tokio $(task_count))

rust_hash_mutex_async_std:
	(cd rust && cargo run --release -p hash_mutex --bin async_std $(task_count))

rust_hash_mutex_rayon:
	(cd rust && cargo run --release -p hash_mutex --bin rayon $(task_count))

rust_hash_mutex_threads:
	(cd rust && cargo run --release -p hash_mutex --bin threads $(thread_count))

rust_hash_mutex_tokio:
	(cd rust && cargo run --release -p hash_mutex --bin tokio $(task_count))

rust_hash_message_threads:
	(cd rust && cargo run --release -p hash_message --bin threads $(thread_count))

rust_hash_message_tokio:
	(cd rust && cargo run --release -p hash_message --bin tokio $(task_count))

go_stat:
	(cd go/running_stat && go run main.go stat.go $(task_count))

go_hash:
	(cd go/hash && go run main.go $(task_count))

net6_stat:
	(cd dotnet/runningStat/net6 && dotnet run $(task_count))

net7_stat:
	(cd dotnet/runningStat/net7 && dotnet run $(task_count))