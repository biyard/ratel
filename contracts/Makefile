TARGET ?= ./scripts/deploy_space_dao.ts
NETWORK ?= localhost
PRIVATE_KEY ?=

SPACE_DAO_ADDR ?= 
REWARD_EXTENSION_ADDR ?= 

SPACE_FACTORY_ADDR ?= 

deploy:
	PRIVATE_KEY=$(PRIVATE_KEY) npx hardhat run $(TARGET) --network $(NETWORK)

build: node_modules
	npx hardhat compile

node:
	npx hardhat node

test:
	npx hardhat test

compile:
	npx hardhat compile