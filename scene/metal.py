from scene_object import scene_object

def metal(albedo):
    return scene_object("metal", {
        "albedo": albedo
        })
