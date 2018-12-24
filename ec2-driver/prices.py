import boto3

client = boto3.client('ec2',region_name='us-east-1')
# ec2.create_instances(ImageId='ami-0ac019f4fcb7cb7e6', MinCount=1, MaxCount=1)

def ask_price(instance_type):
    prices=client.describe_spot_price_history(InstanceTypes=[instance_type],MaxResults=10,ProductDescriptions=['Linux/UNIX (Amazon VPC)'],AvailabilityZone='us-east-1a')
    return prices['SpotPriceHistory'][0]

##############################################################################

instances = [
    { 'name': 'c4.large',    'vCPUs': 2,  'mem': 3.75 },
    { 'name': 'c5.large',    'vCPUs': 2,  'mem': 4 },
    { 'name': 'c4.xlarge',   'vCPUs': 4,  'mem': 7.5 },
    { 'name': 'c5.xlarge',   'vCPUs': 4,  'mem': 8 },
    { 'name': 'c4.2xlarge',  'vCPUs': 8,  'mem': 15 },
    { 'name': 'c5.2xlarge',  'vCPUs': 8,  'mem': 16 },
    { 'name': 'c4.4xlarge',  'vCPUs': 16, 'mem': 30 },
    { 'name': 'c5.4xlarge',  'vCPUs': 16, 'mem': 32 },
    { 'name': 'c4.8xlarge',  'vCPUs': 36, 'mem': 60 },
    { 'name': 'c5.9xlarge',  'vCPUs': 36, 'mem': 72 },
    { 'name': 'c5.18xlarge', 'vCPUs': 72, 'mem': 144 },
    # { 'name': 'c5d.large', 'vCPUs': 2, 'mem': 4 },
    # { 'name': 'c5d.xlarge', 'vCPUs': 4, 'mem': 8 },
    # { 'name': 'c5d.2xlarge', 'vCPUs': 8, 'mem': 16 },
    # { 'name': 'c5d.4xlarge', 'vCPUs': 16, 'mem': 32 },
    # { 'name': 'c5d.9xlarge', 'vCPUs': 36, 'mem': 72 },
    # { 'name': 'c5d.18xlarge', 'vCPUs': 72, 'mem': 144 }
    ]

for instance in instances:
    d = ask_price(instance['name'])
    print("Instance: %s" % instance['name'])
    print("  U$/hour: %s" % d['SpotPrice'])
    print("  U$/vCPU hour: %.4f" % (float(d['SpotPrice']) / instance['vCPUs']))
    print("  mem: %s" % instance['mem'])
    print("  vCPUs: %s" % instance['vCPUs'])

