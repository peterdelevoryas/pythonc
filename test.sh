#!/bin/bash

dir=$1
compiler=$2
runtime=$3

for py in $dir/*.py; do
    [ -f "$py" ] || break
    target="$(basename $py .py)"
    source="$target.py"
    input="$target.in"
    output="$target.out"
    expected="$target.expected"
    assembly="$target.s"
    $compiler $source
    gcc -m32 -g -lm $assembly $runtime -o $target
    cat $input | ./$target > $output
    cat $input | python $source > $expected
    diff -w -B $expected $output
done
