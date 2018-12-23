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

l_w = luminance(image_array)

try:
    l_white = float(sys.argv[2])
except:
    l_white = l_w.max()

print("l_white: %.3f" % l_white)

try:
    a = float(sys.argv[3])
except IndexError:
    a = 0.18
print("a: %.3f" % a)

def log_average_luminance(array):
    n = array.shape[0] * array.shape[1]
    return math.exp(scipy.log(luminance(image_array) + 0.01).sum() / n)

l_wbar = log_average_luminance(image_array)

l = (a / l_wbar) * l_w
l_d = l * (1 + (l / (l_white * l_white))) / (1 + l)
lum_scale = l_d / l_w

image_array = image_array * lum_scale[:,:,scipy.newaxis]

##############################################################################

# linear
image_array[image_array >= 1] = 1
image_array[image_array < 0] = 0
i8_array = scipy.array(image_array * 255, 'u8')
image = Image.fromarray(i8_array.astype('uint8'))
image.show()
image.save(sys.argv[1])
