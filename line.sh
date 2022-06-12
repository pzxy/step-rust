#!/bin/zsh
# shellcheck disable=SC2219
let ret=0
# shellcheck disable=SC2044
# shellcheck disable=SC2006
for line in `find "${1:-"."}" -type f -name "${2:-"*.rs"}"`
do
  # shellcheck disable=SC2006
  let s=`wc -l $line | awk '{print $1}'`
  let ret+=$s
done
echo "$ret"