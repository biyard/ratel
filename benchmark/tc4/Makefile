TEST_FILE ?= tc4.jmx
RESULT_FILE ?= result-tc4.jtl

clean:
	rm -rf build

ec2.clear:
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "rm -rf ~/data ~/report && mkdir ~/data"

ec2.copy:
	scp -i $(SSH_KEY_PATH) $(TEST_FILE) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/

ec2.run: ec2.clear ec2.copy
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "rm -rf /home/ubuntu/$(RESULT_FILE)"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "apache-jmeter-5.6.3/bin/jmeter -n -t $(TEST_FILE) -l /home/ubuntu/$(RESULT_FILE) -e -o ./report"
	scp -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/$(RESULT_FILE) .
	scp -r -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/report .
