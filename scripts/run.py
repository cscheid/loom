#!/usr/bin/env python3

import subprocess
import sys
import tempfile
import click
import time

@click.command()
@click.option('--scene', help='Scene file.')
@click.option('--output', help='Output image.')
@click.option('--samples', default=1, help='Samples per pixel.')
@click.option('--processes', default=4, help='Processes to run.')
@click.option('--height', default=480, help='Image height in pixels.')
def run(scene, output, samples, processes, height):
    """Simple parallel runner for loom."""
    if scene is None:
        print("Expected a scene option")
        exit(1)
    if output is None:
        print("Expected an output option")
        exit(1)
        
    n_procs = processes
    scene_file = scene
    output_image = output
    samples_per_pixel = samples # int(samples / n_procs)
    print("Starting loom's parallel driver.")
    print("  per-process spp: %d" % samples_per_pixel)
    image_height = height
    print("  scene file:", scene)
    print("  output file:", output)
    start = time.time()
    with tempfile.TemporaryDirectory() as td:
        print("Running parallel driver..")
        subprocess.run(['./scripts/parallel_driver.py',
                        str(n_procs),
                        scene,
                        str(image_height),
                        str(samples_per_pixel),
                        td + '/out'])
        subprocess.run(["mv",
                        td + '/out.linear_rgb',
                        output])
    elapsed = time.time() - start
    print("Done. Total runtime: %.3fs" % elapsed)
if __name__ == '__main__':
    run()
