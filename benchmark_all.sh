#!/bin/sh

for i in $(seq 1 25)
do 
  env AOC_BENCHMARK=1 MIMALLOC_LARGE_OS_PAGES=1 MIMALLOC_PAGE_RESET=0 cargo run --bin "$i" --release --quiet
done
