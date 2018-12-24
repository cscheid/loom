import boto3

ec2 = boto3.client('ec2',region_name='us-east-1')

r = ec2.describe_spot_instance_requests()

for req in r["SpotInstanceRequests"]:
    print(req["InstanceId"])
    print("  ", req["LaunchSpecification"]["InstanceType"])
    print("  ", req["State"])
    print("  ", req["Status"])
