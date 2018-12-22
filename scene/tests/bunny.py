#!/usr/bin/env python

from scene import *

scene = Scene()
scene.background = "overhead_light"
scene.camera = camera(
    look_from=[0.0,   0.2,  3.0],
    look_at=[-0.015,   0.113,  0.0],
    vup=[0.0,   1.0,  0.0],
    vfov=3.3,
    aspect=1.6,
    aperture=0.0,
    focus_dist=0.8)

scene.add_object(rectangle(
    bottom_left=[2.0, 0.035, -2.0],
    right=[-4.0, 0.0, 0.0],
    up=[0.0, 0.0, 4.0],
    material=lambertian([0.7, 0.8, 0.9])))

scene.add_object(triangle_mesh(
    file_name="tests/bunny.json",
    material=lambertian([1.0, 0.6, 0.2])))

scene.write()
