#!/usr/bin/env python

from scene import *

scene = Scene()
scene.background = constant([0,0,0])
scene.camera = camera(
    look_from  = [0.0,   1,   5.0],
    look_at    = [0.0,   1,  -1.0],
    vup        = [0.0,   1,   0.0],
    vfov       = 29,
    aspect     = 1,
    aperture   = 0.0,
    focus_dist = 0.8)


path = "/home/cscheid/data/gfx/models/cornell-box"

scene.add_object(triangle_mesh(
    file_name=path + "/cornellbox-backwall.json",
    material=lambertian([0.725, 0.71, 0.68])))

scene.add_object(triangle_mesh(
    file_name=path + "/cornellbox-ceiling.json",
    material=lambertian([0.725, 0.71, 0.68])))

scene.add_object(triangle_mesh(
    file_name=path + "/cornellbox-floor.json",
    material=lambertian([0.725, 0.71, 0.68])))

scene.add_object(triangle_mesh(
    file_name=path + "/cornellbox-leftwall.json",
    material=lambertian([0.63, 0.065, 0.05])))

scene.add_object(triangle_mesh(
    file_name=path + "/cornellbox-rightwall.json",
    material=lambertian([0.14, 0.45, 0.091])))

# scene.add_object(triangle_mesh(
#     file_name=path + "/cornellbox-tallbox.json",
#     material=lambertian([0.725, 0.71, 0.68])))

# scene.add_object(triangle_mesh(
#     file_name=path + "/cornellbox-shortbox.json",
#     material=lambertian([0.725, 0.71, 0.68])))

scene.add_object(triangle_mesh(
    file_name=path + "/cornellbox-light.json",
    material=emitter([0.725 * 17,
                      0.71 * 14,
                      0.68 * 4])))

scene.add_object(sphere(
    center=[-0.55, 0.25, -0.6],
    radius=0.25,
    material=metal([1,1,1])))

scene.add_object(sphere(
    center=[0.0, 0.25, -0.6],
    radius=0.25,
    material=phong([1,1,1], 0.1)))

scene.add_object(sphere(
    center=[0.55, 0.25, -0.6],
    radius=0.25,
    material=lambertian([1,1,1])))

# scene.add_object(sphere(
#     center=[0.5, 0.3, 0.2],
#     radius=0.3,
#     material=glass()))



scene.write()
