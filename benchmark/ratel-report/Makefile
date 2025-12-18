SHELL := /usr/bin/env bash
.SHELLFLAGS := -lc

PRESENCE_BASE ?= http://localhost:3000
PAGE_KEY ?= /spaces/SPACE%23019abfa5-c420-72b2-8220-45f3918b001e/polls/SPACE_POLL%23019ac017-a6bd-74e2-9d01-2f75b0fc300d

STATS_PATH ?= /v3/presence/stats
START_PATH ?= /v3/presence/start
PING_PATH  ?= /v3/presence/ping

REQUIRED_PEAK ?= 2000
CHECK_DURATION_SECS ?= 300

N ?= 2000
WINDOW_SECS ?= 30
PING_INTERVAL_SECS ?= 10

START_CONCURRENCY ?= 60
PING_CONCURRENCY ?= 200

BATCH ?= 200
BATCH_SLEEP_SECS ?= 1

SID_FILE ?= /tmp/presence_sids.txt

JMETER ?= jmeter

jmeter:
	$(JMETER) -n -t presence_loadtest.jmx \
		-JPROTOCOL=https -JDOMAIN=ratel.foundation -JPORT=443 \
		-JPAGE_KEY=/spaces/SPACE%23019abfa5-c420-72b2-8220-45f3918b001e/polls/SPACE_POLL%23019ac017-a6bd-74e2-9d01-2f75b0fc300d \
		-JREQUIRED_PEAK=2000 -JTEST_DURATION_SECS=300

local.reset:
	@curl -sS "$(PRESENCE_BASE)$(STATS_PATH)?reset=true" >/dev/null || true
	@rm -f "$(SID_FILE)"

local.start:
	@PRESENCE_BASE="$(PRESENCE_BASE)" \
	START_PATH="$(START_PATH)" \
	PAGE_KEY="$(PAGE_KEY)" \
	N="$(N)" \
	START_CONCURRENCY="$(START_CONCURRENCY)" \
	BATCH="$(BATCH)" \
	BATCH_SLEEP_SECS="$(BATCH_SLEEP_SECS)" \
	SID_FILE="$(SID_FILE)" \
	python3 scripts/start_users.py

local.ping.once:
	@PRESENCE_BASE="$(PRESENCE_BASE)" \
	PING_PATH="$(PING_PATH)" \
	PAGE_KEY="$(PAGE_KEY)" \
	SID_FILE="$(SID_FILE)" \
	PING_CONCURRENCY="$(PING_CONCURRENCY)" \
	python3 scripts/ping_once.py >/dev/null

local.stats:
	@curl -sS "$(PRESENCE_BASE)$(STATS_PATH)"; echo

local.loadtest: local.reset local.start
	@bash -lc 'set -euo pipefail; \
	  dur="$(CHECK_DURATION_SECS)"; req="$(REQUIRED_PEAK)"; ping_i="$(PING_INTERVAL_SECS)"; \
	  observed=0; \
	  for sec in $$(seq 1 "$$dur"); do \
	    res=$$(curl -sS "$(PRESENCE_BASE)$(STATS_PATH)"); \
	    cur=$$(python3 -c "import json,sys; d=json.loads(sys.stdin.read()); print(int(d.get(\"current\",0)))" <<< "$$res"); \
	    peak=$$(python3 -c "import json,sys; d=json.loads(sys.stdin.read()); print(int(d.get(\"peak\",0)))" <<< "$$res"); \
	    if [ "$$peak" -gt "$$observed" ]; then observed="$$peak"; fi; \
	    if [ $$((sec % ping_i)) -eq 0 ]; then $(MAKE) -s local.ping.once; fi; \
	    if [ $$((sec % 5)) -eq 0 ]; then echo "t=$$sec current=$$cur peak=$$peak observed=$$observed"; fi; \
	    sleep 1; \
	  done; \
	  pass=false; if [ "$$observed" -ge "$$req" ]; then pass=true; fi; \
	  echo "{\"observed_peak\": $$observed, \"required_peak\": $$req, \"pass\": $$pass}" | tee presence_peak_report.json; \
	'
