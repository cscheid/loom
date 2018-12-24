#!/bin/bash

scp $1 ubuntu@${AMAZON_IP}:scene.tar.gz
ssh ubuntu@${AMAZON_IP} "tar xvzf scene.tar.gz"

