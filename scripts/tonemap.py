#!/usr/bin/env python

import scipy
import sys
import math
from PIL import Image

f = sys.stdin.readline().strip().split()

w, h = tuple(int(x) for x in f)
image_array = scipy.zeros((h, w, 3))

for y in range(h):
    for x in range(w):
        image_array[y, x, :] = list(float(v) for v in sys.stdin.readline().strip().split())

##############################################################################

def luminance(array):
    return array[:,:,0] * 0.2126 + \
           array[:,:,1] * 0.7152 + \
           array[:,:,2] * 0.0722

def log_average_luminance(array):
    n = array.shape[0] * array.shape[1]
    return math.exp(scipy.log(l_w).sum() / n)

##############################################################################

l_w = luminance(image_array) + 0.001
try:
    l_white = float(sys.argv[2])
    if l_white == -1:
        l_white = l_w.max()
except:
    l_white = l_w.max()

try:
    a = float(sys.argv[3])
except IndexError:
    a = 0.18

##############################################################################

l_wbar = log_average_luminance(image_array)
l = (a / l_wbar) * l_w
l_d = l * (1 + (l / (l_white * l_white))) / (1 + l)
lum_scale = l_d / l_w
image_array = image_array * lum_scale[:,:,scipy.newaxis]

print("l_white: %.3f" % l_white)
print("a: %.3f" % a)
print("scene's log-average luminance: %.3f" % l_wbar)

##############################################################################

# linear
image_array[image_array >= 1] = 1
image_array[image_array < 0] = 0
i8_array = scipy.array(image_array * 255, 'u8')
image = Image.fromarray(i8_array.astype('uint8'))
image.show()
image.save(sys.argv[3])
