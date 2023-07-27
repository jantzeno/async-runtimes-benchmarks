# Time Helpers
timejson() {
  /usr/bin/time --quiet -a -o ${starting_dir}/json/${1} -f \
'{
   "involuntary_context_switches": %c,
   "real_time_formatted": "%E",
   "real_time_seconds": %e,
   "major_page_faults": %F,
   "avg_total_memory_use_kb": %K,
   "max_resident_set_kb": %M,
   "cpu_percentage": "%P",
   "minor_page_faults": %R,
   "system_time_seconds": %S,
   "user_time_seconds": %U,
   "voluntary_context_switches": %w,
   "file_system_outputs": %O,
   "average_resident_set_kb": %t,
   "file_system_inputs": %I,
   "exit_status": %x
},' ${2}
}

timejsonloop() {
  for (( n = 1; n <= $1; n++)); do
    timejson "$2" "$3"
  done
}

starting_dir=$(pwd)
async=1000000
threads=10000
run_count=10

rust_stat_mutex_async=("async_std" "rayon" "tokio")
rust_stat_mutex_thread=("threads")

rust_stat_message_async=("tokio")
rust_stat_message_thread=("threads")

rust_hash_mutex_async=("async_std" "rayon" "tokio")
rust_hash_mutex_thread=("threads")

rust_hash_message_async=("tokio")
rust_hash_message_thread=("threads")

function rust_benchmark_runner() {
  local params=("$@")
  local bench_function="${params[0]}"
  local max="${params[1]}"
  local benchmarks=("${params[@]:2}")

  for benchmark in "${benchmarks[@]}"; do
    for (( i = 100; i <= $max; i *= 10)); do
      # echo "$bench_function $benchmark $i"
      $bench_function $benchmark $i
    done
  done
}

function simple_benchmark_runner() {
  local bench_function="$1"
  local max="$2"

  for (( i = 100; i <= $max; i *= 10)); do
    # echo "$bench_function $i"
    $bench_function $i
  done
}

function dotnet_benchmark_runner() {
  local bench_function="$1"
  local version="$2"
  local max="$3"

  for (( i = 100; i <= $max; i *= 10)); do
    # echo "$bench_function $version $i"
    $bench_function $version $i
  done
}

function build_java() {
  for build in "${java_build[@]}"; do
    cd java/$build
    javac --release 20 --enable-preview *.java
    cd $starting_dir
  done
}

java_build=("stat" "hash")

# Java
function java_benchmark_runner() {
  for build in "${java_build[@]}"; do
    cd java/$build
      for (( i = 100; i <= ${2}; i *= 10)); do
       timejsonloop $run_count "java_${build}_${1}_${i}.json" "java --enable-preview ${1} ${i}"
      done
    cd $starting_dir
  done
}

# Rust
# Stat Mutex
function rust_stat_mutex() {
  cd rust
  timejsonloop $run_count "rust_stat_mutex_${1}_${2}.json" "cargo run --release -p stat_mutex --bin ${1} ${2}"
  cd $starting_dir
}

# Stat Message
function rust_stat_message() {
  cd rust
  timejsonloop $run_count "rust_stat_message_${1}_${2}.json" "cargo run --release -p stat_message --bin ${1} ${2}"
  cd $starting_dir
}

# Hash Mutex
function rust_hash_mutex() {
  cd rust
  timejsonloop $run_count "rust_hash_mutex_${1}_${2}.json" "cargo run --release -p hash_mutex --bin ${1} ${2}"
  cd $starting_dir
}

# Hash Message
function rust_hash_message() {
  cd rust
  timejsonloop $run_count "rust_hash_message_${1}_${2}.json" "cargo run --release -p hash_message --bin ${1} ${2}"
  cd $starting_dir
}

# Go
# Stat
function go_stat() {
  cd go/running_stat 
  timejsonloop $run_count "go_stat_${1}.json" "go run main.go running_stat.go ${1}"
  cd $starting_dir
}

# Hash
function go_hash() {
  cd go/hash
  timejsonloop $run_count "go_hash_${1}.json" "go run main.go ${1}"
  cd $starting_dir
}

# Dotnet
function dotnet_cleanup() {
  cd dotnet/runningStat/${1}
  dotnet clean
  cd $starting_dir
}

function net_stat() {
  cd dotnet/runningStat/${1}
  timejsonloop $run_count "${1}_stat_${2}.json" "dotnet run ${2}"
  cd $starting_dir
}

# Run becnchmarks
async-runtime-benchmark() {
  echo "Running benchmarks..."
  echo "Rust Benchmarks"
  rust_benchmark_runner rust_stat_mutex $async "${rust_stat_mutex_async[@]}"
  rust_benchmark_runner rust_stat_mutex $threads "${rust_stat_mutex_thread[@]}"
  rust_benchmark_runner rust_stat_message $async "${rust_stat_message_async[@]}"
  rust_benchmark_runner rust_stat_message $threads "${rust_stat_message_thread[@]}"
  rust_benchmark_runner rust_hash_mutex $async "${rust_hash_mutex_async[@]}"
  rust_benchmark_runner rust_hash_mutex $threads "${rust_hash_mutex_thread[@]}"
  rust_benchmark_runner rust_hash_message $async "${rust_hash_message_async[@]}"
  rust_benchmark_runner rust_hash_message $threads "${rust_hash_message_thread[@]}"
  
  echo "Go Benchmarks"
  simple_benchmark_runner go_stat $async
  simple_benchmark_runner go_hash $async

  echo "Dotnet Benchmarks"
  dotnet_cleanup net6
  dotnet_cleanup net7
  dotnet_benchmark_runner net_stat net6 $async
  dotnet_benchmark_runner net_stat net7 $async

  echo "Java Benchmarks"
  build_java
  java_benchmark_runner Threads $async
  java_benchmark_runner VirtualThreads $async
}