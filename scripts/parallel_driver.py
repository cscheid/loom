#!/usr/bin/env python3

from multiprocessing import Pool
import subprocess
import sys

n_procs = int(sys.argv[1])
scene = sys.argv[2]
height = sys.argv[3]
samples = sys.argv[4]
out_file = sys.argv[5]

def render(proc_id):
    print("Rendering %s" % proc_id)
    subprocess.run(["./target/release/loom-render",
                    "-i", scene,
                    "-s", samples,
                    "-h", height,
                    "-p",
                    "-o", "%s-%s" % (out_file, proc_id)])

if __name__ == "__main__":
    with Pool(n_procs) as p:
        p.map(render, list(range(n_procs)))
