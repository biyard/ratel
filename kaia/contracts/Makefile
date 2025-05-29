DOMAIN ?= incheonheroes.world
id ?= 1

node_modules:
	npm install

.PHONY: test
test:
	@npx hardhat test

.PHONY: build help deploy
build: node_modules
	@npx hardhat compile

help:
	node cli.js -h

list:
	node cli.js list

deploy:
	@echo "Deploying IncheonContents contract..."
	$(DEPLOY_ENVS) node cli.js deploy Membership $(CLI_CONTRACT_OWNER_ADDR) "Membership" "M" 'https://$(DOMAIN)/metadata/json/$(id).json'

