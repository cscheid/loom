#!/usr/bin/env python3

import subprocess
import sys
import tempfile
import click
import time
import scipy
from scipy.stats import linregress

def regress(runtimes, spp, height, processes):
    # our (very simplistic!) model for runtime is:
    # t(p) = c + k * p
    #  c is a warmup constant
    #  k is average time per pixel
    #  spp is samples per pixel
    runtimes = scipy.array(runtimes)
    result = linregress(runtimes[:,0], runtimes[:,1])
    k = result[0]
    c = result[1]
    h = float(height)
    p = float(processes)
    t = (k * h * h * spp / p + c)
    return t

@click.command()
@click.option('--scene', help='Scene file.')
@click.option('--height', help='Image height in pixels in full render.')
@click.option('--samples', help='Samples per pixel in full render.')
@click.option('--processes', help='Number of processes in full render.')
@click.option('--output', default=None, help='Unused option to match interface of run.py.')
def run(scene, height, samples, processes, output):
    """Estimates runtime of full render by running a tiny noisy version."""
    if scene is None:
        print("Expected a --scene option")
        exit(1)
    if height is None:
        print("Expected a --height option")
        exit(1)
    if samples is None:
        print("Expected a --samples option")
        exit(1)
    if processes is None:
        print("Expected a --processes option")
        exit(1)

    scene_file = scene
    image_height = height
    interval = 1
    with tempfile.TemporaryDirectory() as td:
        i = 1
        size = 10
        runtimes = []
        elapsed = 0
        npixels = 0
        while elapsed < 1:
            print("\r" + " " * 80 + "\r", end='')
            print("Running probe %d... " % i, end='')
            start = time.time()
            print("%s pixels in %.3fs. " % (npixels, elapsed), end='')
            if len(runtimes) > 2:
                print("Guess: %.3fs" % regress(runtimes, int(samples), height, processes), end='')
            sys.stdout.flush()
            subprocess.run(['./scripts/parallel_driver.py',
                            "1",
                            scene,
                            str(int(size)),
                            "1",
                            "1",
                            td + '/out'], stdout=subprocess.DEVNULL)
            elapsed = time.time() - start
            runtimes.append((npixels, elapsed))
            i += 1
            size *= 1.5
            npixels = (int(size) ** 2) # wrong! we need to use camera information
        print()
        print("Estimated total time: %.3fs" % regress(runtimes, int(samples), height, processes))

if __name__ == '__main__':
    run()
