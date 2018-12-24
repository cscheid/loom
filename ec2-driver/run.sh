#!/bin/bash

source ~/venv/bin/activate
NCPUS=`lscpu | grep -E '^Thread|^Core|^Socket|^CPU\(' | head -n1 | cut -d':' -f2 | xargs`
echo Samples: $SAMPLES
echo Height: $HEIGHT
echo CPUs: $NCPUS
./scripts/run.py --scene scene/input.json --samples $SAMPLES --processes $NCPUS --height $HEIGHT --output scene/output.rgb_linear
