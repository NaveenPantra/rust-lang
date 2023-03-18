#!/usr/bin/env bash


OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello World" > $OUTDIR/hello1.txt
echo "Hello" "World" > $OUTDIR/hello2.txt
echo -n "Hello World" > $OUTDIR/hello3.txt
echo -n "Hello" "World" > $OUTDIR/hello4.txt
