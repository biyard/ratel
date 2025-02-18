ENV ?= local

ifeq ("$(ENV)","local")
	MODE = development
	FLAGS =
else
	MODE = production
	FLAGS = -d false
endif

node_modules:
	@npm install
	@npm install --save-dev webpack webpack-cli

.PHONY: build
build: node_modules clean
	@webpack --mode $(MODE) $(FLAGS)

clean:
	@rm -rf build
