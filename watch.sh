#!/bin/bash

# This may be changed to a more specific folder when developing selected areas
WATCH_FOLDER=src

# In this mode, the markdown files are consolidated and HTML is generated via markdown
PUBLISH=true

# This builds rust2md upon startup
DEV_MODE=true

# This is the location of the rust2md executable
RUST2MD="rust2md/target/release/rust2md"

if [[ ! -e $RUST2MD || DEV_MODE ]]; then
    cd rust2md
    cargo build --release
    cd ..
fi

mkdir -p docs
while true
do
  $RUST2MD < src/control_flow/src/main.rs > docs/control_flow.md
  $RUST2MD < src/data_types/src/main.rs > docs/data_types.md
  echo "Waiting for changes. Press CTRL+C to quit."
  if [[ PUBLISH ]]; then
    cd docs
    cat ../md/header.md > index.md
    cat data_types.md >> index.md
    cat control_flow.md >> index.md
    pandoc --standalone --toc index.md -o index.html
    cd ..
  fi
  inotifywait -r -e modify $WATCH_FOLDER 
done
