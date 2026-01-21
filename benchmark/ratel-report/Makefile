SHELL := /usr/bin/env bash
.SHELLFLAGS := -lc

JMETER ?= jmeter

PROTOCOL ?= https
DOMAIN ?= dev.ratel.foundation
PORT ?= 443

PAGE_PATH ?= /spaces/SPACE%23019abfa5-c420-72b2-8220-45f3918b001e/polls/SPACE_POLL%23019ac017-a6bd-74e2-9d01-2f75b0fc300d

API_PATH ?=
API_METHOD ?= GET
API_BODY ?=

USERS ?= 2000
RAMP_SECS ?= 60
DURATION_SECS ?= 300

POLL_INTERVAL_SECS ?= 10

OUT_DIR ?= report
JTL ?= results.jtl

clean:
	rm -f $(JTL)
	rm -rf $(OUT_DIR)

jmeter.dev:
	$(JMETER) -n -t ratel_web_loadtest.jmx \
		-JPROTOCOL=$(PROTOCOL) -JDOMAIN=$(DOMAIN) -JPORT=$(PORT) \
		-JPAGE_PATH="$(PAGE_PATH)" \
		-JAPI_PATH="$(API_PATH)" -JAPI_METHOD="$(API_METHOD)" -JAPI_BODY='$(API_BODY)' \
		-JUSERS=$(USERS) -JRAMP_SECS=$(RAMP_SECS) -JDURATION_SECS=$(DURATION_SECS) \
		-JPOLL_INTERVAL_SECS=$(POLL_INTERVAL_SECS) \
		-l $(JTL) -e -o $(OUT_DIR)
