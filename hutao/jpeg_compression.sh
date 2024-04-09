#!/bin/bash

# Using this utility:
# https://github.com/tjko/jpegoptim
#
# Sets all the files ending in .jpg and .jpeg in the current directory with an image
# quality of 70.
echo "
Optimizing the .JPEG/.JPG images. This could take a while depending on how many
images with those endings are inside the current directory."
jpegoptim ./*.jpg -m 70
jpegoptim ./*.jpeg -m 70
