from scene_object import scene_object

sky = "sky"
overhead_light = "overhead_light"

def constant(color):
    return scene_object("constant", { "color": color })
