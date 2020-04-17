#! /usr/bin/env bash

RUSTFLAGS="-C target-cpu=native" cargo build --release

cat assets/Sudoku.csv \
| tail -n +2 \
| cut -d ';' --fields=1,2,3 \
| tr \; ' ' \
| xargs -n 3 -L 1 bash -c 'echo Executing puzzle $0 with difficulty $1; ./target/release/sudoku_solver $2' \
> assets/solutions.txt

