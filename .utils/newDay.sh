#!/usr/bin/env bash

DAY=$1
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cargo aoc input -d $DAY

cat $SCRIPT_DIR/dayTemplate.rs | sed "s/dayx/day$DAY/g" > "$SCRIPT_DIR/../src/solutions/day$DAY.rs"
echo "pub mod day$DAY;" >> "$SCRIPT_DIR/../src/solutions/mod.rs"
