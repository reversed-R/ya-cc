#!/bin/bash

for FILE in ./c/*; do
  echo "$FILE"
  cargo run --manifest-path ../Cargo.toml $FILE
done

