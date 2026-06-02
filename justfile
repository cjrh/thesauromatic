# thesauromatic task runner. Run `just` to list recipes.

# Show available recipes.
default:
    @just --list

# Run the full test suite (unit + end-to-end regression tests).
test:
    cargo test --release

# Build the optimized release binary.
build:
    cargo build --release

# Regenerate the golden output files from the current release binary.
# Use this only when the word data is intentionally changed.
bless: build
    #!/usr/bin/env bash
    set -euo pipefail
    bin=target/release/thesauromatic
    mkdir -p tests/golden
    declare -A words=(
        [deluge]=deluge.txt
        [happy]=happy.txt
        [construction]=construction.txt
        [stamp]=stamp.txt
        ["a cappella"]=a_cappella.txt
        [blahblahnotaword]=blahblahnotaword.txt
    )
    for w in "${!words[@]}"; do
        "$bin" "$w" > "tests/golden/${words[$w]}"
        echo "blessed ${words[$w]} ($w)"
    done

# Benchmark end-to-end runtime with hyperfine (requires `hyperfine`).
bench word="deluge": build
    hyperfine -N --warmup 50 'target/release/thesauromatic {{word}}'
