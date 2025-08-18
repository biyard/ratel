EC2_AGENT_INSTANCE_ID ?= $(shell aws ec2 describe-instances --filters "Name=tag:Name,Values=tps-testing-agent" | jq '.Reservations | map(select(.Instances[].State.Name!="terminated")) | map(.Instances[0].InstanceId)[]' | tr -d \")
EC2_AGENT_PUBLIC_IP ?= $(shell aws ec2 describe-instances --filters "Name=tag:Name,Values=tps-testing-agent" | jq '.Reservations | map(select(.Instances[].State.Name!="terminated")) | map(.Instances[0].NetworkInterfaces[0].Association.PublicIp)[]' | tr -d \")
SSH_KEY_PATH ?= ~/.ssh/biyard-new.pem
TEST ?= 12-concurrency
TEST_FILE ?= $(TEST).jmx
RESULT_FILE ?= $(TEST).jtl

ec2.setup:
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "sudo apt update"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "sudo apt install -y openjdk-8-jdk"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "wget https://dlcdn.apache.org//jmeter/binaries/apache-jmeter-5.6.3.tgz"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "tar -xvzf apache-jmeter-5.6.3.tgz"

ec2.start.%:
	aws ec2 start-instances --instance-ids $* > /dev/null

.PHONY: ec2.start
ec2.start: $(patsubst %,ec2.start.%,$(EC2_AGENT_INSTANCE_ID))

ec2.stop.%:
	aws ec2 stop-instances --instance-ids $* > /dev/null

.PHONY: ec2.stop
ec2.stop: $(patsubst %,ec2.stop.%,$(EC2_AGENT_INSTANCE_ID))

ec2.shell.%:
	@echo "ssh -i $(SSH_KEY_PATH) ubuntu@$*"

.PHONY: ec2.shell ec2.copy ec2.run
ec2.shell: $(patsubst %,ec2.shell.%,$(EC2_AGENT_PUBLIC_IP))

ec2.copy:
	scp -i $(SSH_KEY_PATH) $(TEST_FILE) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/

ec2.run: ec2.copy
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "rm -rf /home/ubuntu/$(RESULT_FILE)"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "apache-jmeter-5.6.3/bin/jmeter -n -t $(TEST_FILE) -l /home/ubuntu/$(RESULT_FILE)"
	scp -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/$(RESULT_FILE) .

ec2.list:
	echo "Instance ID: $(EC2_AGENT_INSTANCE_ID)"
	echo "Public IP: $(EC2_AGENT_PUBLIC_IP)"