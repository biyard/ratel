SHELL := /usr/bin/env bash
.SHELLFLAGS := -lc

JMETER ?= jmeter
JMX_TEMPLATE ?= poll_session_response.jmx
JMX_GEN ?= .poll_session_response.gen.jmx

PROTOCOL ?= https
DOMAIN   ?= dev.ratel.foundation
PORT     ?= 443

TEST_DURATION_SECS ?= 300
TARGET_TPS ?= 3000
TPS_FUDGE_PCT ?= 2
THREADS ?= 1000
RAMP_SECS ?= 5

SPACE_ID_ENC ?=
POLL_ID_ENC  ?=
VERIFY_CODE ?= 000000
USER_PASSWORD ?= 0x1111
QUESTION_ID ?= 1

RESULTS ?= results.jtl
REPORT_DIR ?= report

.PHONY: jmeter jmeter.report jmeter.smoke clean gen check-env

gen:
	@TPM=$$(( ($(TARGET_TPS) * 60 * (100 + $(TPS_FUDGE_PCT))) / 100 )); \
	sed -e "s/__THREADS__/$(THREADS)/g" \
	    -e "s/__RAMP_SECS__/$(RAMP_SECS)/g" \
	    -e "s/__TEST_DURATION_SECS__/$(TEST_DURATION_SECS)/g" \
	    -e "s/__TPM__/$$TPM/g" \
	    "$(JMX_TEMPLATE)" > "$(JMX_GEN)"

check-env:
	@test -n "$(SPACE_ID_ENC)" || (echo "Missing SPACE_ID_ENC" >&2; exit 1)
	@test -n "$(POLL_ID_ENC)"  || (echo "Missing POLL_ID_ENC" >&2; exit 1)

jmeter: check-env gen
	$(JMETER) -n -t $(JMX_GEN) -l $(RESULTS) \
		-JPROTOCOL=$(PROTOCOL) -JDOMAIN=$(DOMAIN) -JPORT=$(PORT) \
		-JSPACE_ID_ENC='$(SPACE_ID_ENC)' -JPOLL_ID_ENC='$(POLL_ID_ENC)' \
		-JVERIFY_CODE='$(VERIFY_CODE)' -JUSER_PASSWORD='$(USER_PASSWORD)' -JQUESTION_ID='$(QUESTION_ID)' \
		-Jhttpclient4.max_conn_total=20000 -Jhttpclient4.max_conn_per_host=20000 \
		-Jhttpclient4.validate_after_inactivity=0 -Jhttpclient4.idletimeout=0

jmeter.report: clean check-env gen
	$(JMETER) -n -t $(JMX_GEN) -l $(RESULTS) -e -o $(REPORT_DIR) \
		-JPROTOCOL=$(PROTOCOL) -JDOMAIN=$(DOMAIN) -JPORT=$(PORT) \
		-JSPACE_ID_ENC='$(SPACE_ID_ENC)' -JPOLL_ID_ENC='$(POLL_ID_ENC)' \
		-JVERIFY_CODE='$(VERIFY_CODE)' -JUSER_PASSWORD='$(USER_PASSWORD)' -JQUESTION_ID='$(QUESTION_ID)' \
		-Jhttpclient4.max_conn_total=20000 -Jhttpclient4.max_conn_per_host=20000 \
		-Jhttpclient4.validate_after_inactivity=0 -Jhttpclient4.idletimeout=0

jmeter.smoke:
	$(MAKE) jmeter THREADS=2 TARGET_TPS=2 TEST_DURATION_SECS=10 RAMP_SECS=1 TPS_FUDGE_PCT=0 \
		SPACE_ID_ENC='$(SPACE_ID_ENC)' POLL_ID_ENC='$(POLL_ID_ENC)' \
		VERIFY_CODE='$(VERIFY_CODE)' USER_PASSWORD='$(USER_PASSWORD)' QUESTION_ID='$(QUESTION_ID)'

clean:
	@rm -rf $(RESULTS) $(REPORT_DIR) $(JMX_GEN)
