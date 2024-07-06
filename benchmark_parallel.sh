#!/bin/bash

# Start your HTTP router in the background
./target/release/rusttp &

# Give the server some time to start
sleep 2

# Define the list of URLs to benchmark
urls=(
    "http://localhost:8000/whoami"
    "http://localhost:8000/page?view=hello-world"
    "http://localhost:8000/page?view=universe-too"
    "http://localhost:8000/page?view=rust"
    "http://localhost:8000/say-hi"
    "http://localhost:8000/nonexistent"
)

# Function to run the wrk command
run_wrk() {
    local url=$1
    if [[ "$url" == "http://localhost:8080/say-hi" ]]; then
        echo "Benchmarking $url (POST)"
        wrk -t12 -c400 -d60s -s post.lua "$url"
    else
        echo "Benchmarking $url"
        wrk -t12 -c400 -d60s "$url"
    fi
}

# Export the function and run the benchmarks concurrently
export -f run_wrk

# Run benchmarks concurrently
printf "%s\n" "${urls[@]}" | xargs -n 1 -P 6 bash -c 'run_wrk "$@"' _

# Kill the HTTP router process after
kill $(jobs -p)
