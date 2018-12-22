from scene_object import scene_object

def rectangle(bottom_left, right, up, material):
    return scene_object("rectangle",
                        {"bottom_left": bottom_left,
                         "right": right,
                         "up": up,
                         "material": material})
