from background import *
from camera import *
from emitter import *
from metal import *
from rectangle import *
from sphere import *
from lambertian import *
from phong import *
from triangle_mesh import *
from materials import *

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
            
