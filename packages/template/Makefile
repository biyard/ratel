generate:
	@if [ -z "$(NAME)" ]; then echo "Usage: make generate NAME=<package-name>"; exit 1; fi
	cargo generate --path . --name $(NAME) --destination ../
