from camera import *
from rectangle import *
from lambertian import *
from triangle_mesh import *

import json

class Scene:
    def __init__(self):
        self.camera = None
        self.object_list = []
        self.background = None
        
    def add_object(self, obj):
        self.object_list.append(obj)

    def write(self):
        print(json.dumps({
            "background": self.background,
            "camera": self.camera,
            "object_list": self.object_list
            }))
            
