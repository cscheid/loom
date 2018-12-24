from scene_object import scene_object

def sphere(center, radius, material):
    return scene_object("sphere",
                        {"center": center,
                         "radius": radius,
                         "material": material})
