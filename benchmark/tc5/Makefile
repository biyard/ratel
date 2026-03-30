WEB_HOST ?= dev.ratel.foundation
WEB_PORT ?= 443
WEB_PROTOCOL ?= https
SPACE_ID ?= replace-with-space-id
SPACE_PAGE ?= dashboard

NUM_THREADS ?= 4000
RAMP_TIME ?= 30
DURATION_SECONDS ?= 330
CONNECT_TIMEOUT_MS ?= 30000
RESPONSE_TIMEOUT_MS ?= 30000

TEST_FILE ?= tc5.jmx
RESULT_FILE ?= result-tc5.jtl
REPORT_DIR ?= report
JMETER_BIN ?= jmeter

JMETER_PROPS ?= \
	-Jweb_host=$(WEB_HOST) \
	-Jweb_port=$(WEB_PORT) \
	-Jweb_protocol=$(WEB_PROTOCOL) \
	-Jspace_id=$(SPACE_ID) \
	-Jspace_page=$(SPACE_PAGE) \
	-Jnum_threads=$(NUM_THREADS) \
	-Jramp_time=$(RAMP_TIME) \
	-Jduration_seconds=$(DURATION_SECONDS) \
	-Jconnect_timeout_ms=$(CONNECT_TIMEOUT_MS) \
	-Jresponse_timeout_ms=$(RESPONSE_TIMEOUT_MS)

clean:
	rm -rf build

local.run:
	rm -rf $(REPORT_DIR) $(RESULT_FILE)
	$(JMETER_BIN) -n -t $(TEST_FILE) $(JMETER_PROPS) -l $(RESULT_FILE) -e -o ./$(REPORT_DIR)

ec2.clear:
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "rm -rf ~/data ~/report && mkdir ~/data"

ec2.copy:
	scp -i $(SSH_KEY_PATH) $(TEST_FILE) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/

ec2.run: ec2.clear ec2.copy
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) curl -i $(WEB_PROTOCOL)://$(WEB_HOST):$(WEB_PORT) > /dev/null
	sleep 5
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "rm -rf /home/ubuntu/$(RESULT_FILE)"
	ssh -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP) "apache-jmeter-5.6.3/bin/jmeter -n -t $(TEST_FILE) $(JMETER_PROPS) -l /home/ubuntu/$(RESULT_FILE) -e -o ./report"
	scp -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/$(RESULT_FILE) .
	scp -r -i $(SSH_KEY_PATH) ubuntu@$(EC2_AGENT_PUBLIC_IP):/home/ubuntu/report .
