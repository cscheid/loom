#!/usr/bin/env python3

# from https://people.cs.clemson.edu/~dhouse/courses/405/docs/brief-obj-file-format.html
# only supporting faces and vertices for now.

import sys
import json

f = open(sys.argv[1])

verts = []
indices = []

for l in f:
    l = l.strip().split()
    if l == []:
        continue
    elif l[0][0] == '#':
        continue
    elif l[0] == 'v':
        verts.append(list(float(i) for i in l[1:]))
    elif l[0] == 'f':
        ixs = list(int(i) for i in l[1:])
        def add(i):
            if i > 0:
                indices.append(i-1) # one-index? idiots.
            elif i < 0:
                indices.append(len(verts)+i)
        # fans out non-triangles
        for t in range(1, len(ixs)-1):
            add(ixs[0])
            add(ixs[t])
            add(ixs[t+1])
    else:
        sys.stderr.write("%s elements are not supported.\n" % l[0])

print(json.dumps({
    "vertices": verts,
    "indices": indices
    }))
