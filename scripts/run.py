#!/usr/bin/env python3

import subprocess
import sys
import tempfile
import click
import time

# n_procs = 4
# scene_file = sys.argv[1]
# output_image = sys.argv[2]
# samples_per_pixel = 10
# image_height = 480
# interval = 10

@click.command()
@click.option('--scene', help='Scene file.')
@click.option('--output', help='Output image.')
@click.option('--linear/--no-linear', default=False, help='if set, write linear image to be tone-mapped later.')
@click.option('--samples', default=1, help='Samples per pixel.')
@click.option('--processes', default=4, help='Processes to run.')
@click.option('--height', default=480, help='Image height in pixels.')
@click.option('--partial', default=-1, help='Produce partial output every this many samples.')
def run(scene, output, linear, samples, processes, height, partial):
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
    samples_per_pixel = int(samples / n_procs)
    print("Starting loom's parallel driver.")
    print("  per-process spp: %d" % samples_per_pixel)
    print("  overall spp: %d" % (samples_per_pixel * n_procs))
    image_height = height
    interval = partial
    if interval == -1:
        interval = samples_per_pixel
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
                        str(interval),
                        td + '/out'])
        print("Combining summaries into output image")
        if linear:
            cmd = ["./target/release/loom-combine-linear"]
        else:
            cmd = ["./target/release/loom-combine"]
        ppm_out = td + '/out'
        cmd.append(ppm_out) # output_image)
        for i in range(n_procs):
            cmd.append(td + ('/out-%d.bincode' % i))
        subprocess.run(cmd)
        if linear:
            print("Copying linear_rgb file...")
            subprocess.run(["cp", ppm_out + ".linear_rgb", output_image])
        else:
            print("Converting to PNG.")
            subprocess.run(["convert", ppm_out + ".ppm", output_image])
    elapsed = time.time() - start
    print("Done. Total runtime: %.3fs" % elapsed)
if __name__ == '__main__':
    run()
