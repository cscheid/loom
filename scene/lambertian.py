from scene_object import scene_object

def lambertian(albedo):
    return scene_object("lambertian", {"albedo": albedo})
