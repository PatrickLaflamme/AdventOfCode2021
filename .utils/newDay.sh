#!/usr/bin/env bash

DAY=$1
YEAR="2021"
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cargo aoc input -d $DAY

cat $SCRIPT_DIR/dayTemplate.rs | sed "s/dayx/day$DAY/g" > "$SCRIPT_DIR/../src/solutions/day$DAY.rs"
echo "pub mod day$DAY;" >> "$SCRIPT_DIR/../src/solutions/mod.rs"

title=`curl -s https://adventofcode.com/$YEAR/day/$DAY | grep Day\ $DAY: | sed -r 's/.*--- (Day.*) ---.*/\1/g'`
echo "- $title [![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/PatrickLaflamme/a054aa6c1453da6f3126d12b4d59ff59/raw/benchmark-aoc-$YEAR-day-$DAY-part-1.json) [A](https://github.com/PatrickLaflamme/AdventOfCode$YEAR/blob/master/src/solutions/day$DAY.rs#L29) | ![badge](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/PatrickLaflamme/a054aa6c1453da6f3126d12b4d59ff59/raw/benchmark-aoc-$YEAR-day-$DAY-part-2.json) [B](https://github.com/PatrickLaflamme/AdventOfCode$YEAR/blob/master/src/solutions/day$DAY.rs#L49)]" >> "$SCRIPT_DIR/../README.md"