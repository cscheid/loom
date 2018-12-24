from scene_object import scene_object

def phong(albedo, glossiness):
    return scene_object("phong", {
        "albedo": albedo,
        "glossiness": glossiness
        })
