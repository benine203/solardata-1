#!/bin/bash

IN="$1"
OUT="$2"

convert -density 300 "$IN" -quality 100 "$OUT"
