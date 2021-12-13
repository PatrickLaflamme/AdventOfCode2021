#! /usr/bin/bash
function displaytime {
  local T=$1
  local S=$((T/1000/1000/1000))
  local MS=$((T/1000/1000))
  local US=$((T/1000))
  local NS=$((T))
  (( $MS > 1000 )) && printf '%.3fs\n' $S
  (( $MS <= 1000 && $US > 1000 )) && printf '%.3fms\n' $MS
  (( $US <= 1000 && $NS > 1000 )) && printf '%.3fµs\n' $US
  (( $NS < 1000 ))     && printf '%.3fns\n' $NS
}

displaytime $1