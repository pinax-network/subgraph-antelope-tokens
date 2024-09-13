.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph create --node http://localhost:8020 eos_tokens
	graph deploy --node=http://localhost:8020 eos_tokens

.PHONY: publish
publish:
	graph publish --subgraph-id ???
