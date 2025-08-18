API_BASE_URL = "https://api.dev.ratel.foundation"
SIGN_DOMAIN = "dev.ratel.foundation"
TEST_USER_PASSWORD ?= "0x4147194710b8e32750bedec1a1a33fb4575ea9510c703863a746c71a1e939bdf"

TARGET ?= "tc1"

EC2_AGENT_INSTANCE_ID ?= $(shell aws ec2 describe-instances --filters "Name=tag:Name,Values=benchmark-agent" | jq '.Reservations | map(select(.Instances[].State.Name!="terminated")) | map(.Instances[0].InstanceId)[]' | tr -d \")
EC2_AGENT_PUBLIC_IP ?= $(shell aws ec2 describe-instances --filters "Name=tag:Name,Values=benchmark-agent" | jq '.Reservations | map(select(.Instances[].State.Name!="terminated")) | map(.Instances[0].NetworkInterfaces[0].Association.PublicIp)[]' | tr -d \")
SCHEDULER ?= single
SSH_KEY_PATH ?= ~/.ssh/biyard-new.pem


run:
	cd ./$(TARGET) && API_BASE_URL=$(API_BASE_URL) SIGN_DOMAIN=$(SIGN_DOMAIN) TEST_USER_PASSWORD=$(TEST_USER_PASSWORD) make run

clean:
	rm -rf build

ec2.clean:
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "rm -rf ~/data && mkdir ~/data"

ec2.setup:
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "sudo apt update"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "sudo apt install -y openjdk-8-jdk"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "wget https://dlcdn.apache.org//jmeter/binaries/apache-jmeter-5.6.3.tgz"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "tar -xvzf apache-jmeter-5.6.3.tgz"
	
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "sudo apt install liblmdb-dev -y"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) 'echo "* - nofile 1000000" | sudo tee -a /etc/security/limits.conf'
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) 'echo "* - memlock 10000000" | sudo tee -a /etc/security/limits.conf'
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) 'echo "* - nproc 999999" | sudo tee -a /etc/security/limits.conf'

ec2.start:
	aws ec2 start-instances --instance-ids $(EC2_AGENT_INSTANCE_ID) > /dev/null

ec2.stop:
	aws ec2 stop-instances --instance-ids $(EC2_AGENT_INSTANCE_ID) > /dev/null

ec2.run:
	cd ./$(TARGET) && SSH_KEY_PATH=$(SSH_KEY_PATH) EC2_AGENT_PUBLIC_IP=$(EC2_AGENT_PUBLIC_IP) EC2_AGENT_INSTANCE_ID=$(EC2_AGENT_INSTANCE_ID) make ec2.run

ec2.shell:
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP)

ec2.list:
	echo "Instance ID: $(EC2_AGENT_INSTANCE_ID)"
	echo "Public IP: $(EC2_AGENT_PUBLIC_IP)"