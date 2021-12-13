#!/usr/bin/env bash

DAY=$1
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

find . -path  $SCRIPT_DIR/../\*/new/estimates.json -print0 | xargs -0 cat | jq '.Mean.point_estimate' | awk '{s+=$1}END{print s/1000000}'