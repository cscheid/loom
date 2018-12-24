#!/bin/sh
IP=$1
./ec2-driver/install.sh $IP
ssh ubuntu@${IP} "./ec2-driver/boot.sh"
