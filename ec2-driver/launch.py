import boto3

ec2 = boto3.client('ec2',region_name='us-east-1')

instance_type = 'c4.large'
max_price = '0.10'
ami = "ami-0d2505740b82f7948" # ubuntu 18.04

response = ec2.request_spot_instances(
    DryRun=False,
    SpotPrice=max_price,
    ClientToken='string',
    InstanceCount=1,
    Type='one-time',
    LaunchSpecification={
        'ImageId': ami,
        'KeyName': 'cscheid',
        'InstanceType': instance_type,
        'Placement': {
            'AvailabilityZone': 'us-east-1f',
        },
        'BlockDeviceMappings': [],
        'EbsOptimized': True,
        'NetworkInterfaces': [
        {
            'DeviceIndex': 0,
            'SubnetId' : 'subnet-fb4989f4',
            'Groups': [
                'sg-0261b63a948b7d93d'
            ],
            'AssociatePublicIpAddress': True            
        }]
        }
)

print(response)

