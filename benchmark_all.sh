#!/bin/sh

for i in $(seq 1 25)
do 
  env AOC_BENCHMARK=1 cargo run --bin "$i" --release --quiet
done
