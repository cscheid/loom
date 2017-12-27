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
    if l[0] == '#':
        continue
    elif l[0] == 'v':
        verts.append(list(float(i) for i in l[1:]))
    elif l[0] == 'f':
        ixs = list(int(i) for i in l[1:])
        fixed_ixs = []
        if len(ixs) != 3:
            sys.stderr.write("Only triangles supported.\n")
            sys.exit(1)
        for i in ixs:
            if i > 0:
                fixed_ixs.append(i-1) # one-index? idiots.
            elif i < 0:
                fixed_ixs.append(len(verts)+i)
        indices.extend(fixed_ixs)
    else:
        sys.stderr.write("%s elements are not supported.\n" % l[0])

print(json.dumps({
    "vertices": verts,
    "indices": indices
    }))
