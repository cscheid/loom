#!/bin/sh
make dist
IP=$1
if [ "$IP" = "" ]; then
    echo Need destination IP.
    exit 1
fi
scp loom-dist.tar.gz ubuntu@${IP}:
ssh ubuntu@${IP} "tar xvzf loom-dist.tar.gz"

