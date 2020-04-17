#! /usr/bin/env bash
set -ea

if [[ $1 = "csv" ]]; then
    cmd='echo -n $0,$1, ; ./target/release/sudoku_solver $2 csv'
else
    cmd='echo Executing puzzle $0 with difficulty $1 ; ./target/release/sudoku_solver $2'
fi

RUSTFLAGS="-C target-cpu=native" cargo build --release

cat assets/Sudoku.csv \
| tail -n +2 \
| cut -d ';' --fields=1,2,3 \
| tr \; ' ' \
| xargs -n 3 -L 1 bash -c "$cmd" \
> assets/solutions.txt
