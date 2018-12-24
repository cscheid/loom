#!/bin/bash

# the double install of scipy is silly, but w/e

cd
sudo apt-get update -y
sudo apt-get install -y python3-venv python3-scipy

python3 -m venv venv
. ./venv/bin/activate
pip install click
pip install scipy

