#!/bin/sh

IP=$1
echo Samples: $2
echo Height: $3
ssh ubuntu@${IP} "SAMPLES=$2 HEIGHT=$3 ./ec2-driver/estimate.sh"
