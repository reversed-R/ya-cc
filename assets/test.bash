#!/bin/bash

FILE=$1

# for FILE in ./c/*; do
#   echo "$FILE"
#   cargo run --manifest-path ../Cargo.toml $FILE
# done

cargo run --manifest-path ../Cargo.toml $FILE > $FILE.s
gcc $FILE.s -o a.out && ./a.out
