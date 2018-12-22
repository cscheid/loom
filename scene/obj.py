import subprocess
import json

def obj(filename):
    proc = subprocess.run(["./scripts/obj_to_json.py", filename],
                          output=subprocess.PIPE)
    return json.load(proc.stdout)


