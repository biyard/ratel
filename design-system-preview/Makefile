ENV ?= local
DYNAMO_TABLE_PREFIX ?= "ratel-$(ENV)"
PORT ?= 6000

BUILD_ENV ?= ENV=$(ENV) \
			DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
			PORT=$(PORT)

run:
	$(BUILD_ENV) dx serve --web
