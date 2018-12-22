from scene_object import scene_object

def triangle_mesh(file_name, material):
    return scene_object("triangle_mesh",
                        {"file_name": file_name,
                         "material": material})
