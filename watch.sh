#!/bin/bash

cd rust2md
cargo clean
cargo build --release
cd ..

RUST2MD="rust2md/target/release/rust2md"

if [[ ! -e $RUST2MD ]]; then
    cd rust2md
    cargo build --release
    cd ..
fi

mkdir -p docs
while true
do
  $RUST2MD < src/data_types/src/main.rs > docs/data_types.md
  echo "Waiting for changes. Press CTRL+C to quit."
  inotifywait -r -e modify src 
done
