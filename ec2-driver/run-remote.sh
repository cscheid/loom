#!/bin/sh

IP=$1
echo Samples: $2
echo Height: $3
echo Local name: $4
ssh ubuntu@${IP} "SAMPLES=$2 HEIGHT=$3 ./ec2-driver/run.sh"
ssh -C ubuntu@${IP} "cat scene/output.rgb_linear" > $4
