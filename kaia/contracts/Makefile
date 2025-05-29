DOMAIN ?= incheonheroes.world

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
