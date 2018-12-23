#!/usr/bin/env python3

from multiprocessing import Pool
import subprocess
import sys

n_procs = int(sys.argv[1])
scene = sys.argv[2]
height = sys.argv[3]
samples = sys.argv[4]
out_file = sys.argv[5]

subprocess.run(["./target/release/loom-render",
                "-i", scene,
                "-s", samples,
                "-h", height,
                "-n", str(n_procs),
                "-p",
                "-o", out_file])
