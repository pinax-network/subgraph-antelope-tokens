.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node http://localhost:8020 kylin_tokens
	graph deploy --node=http://localhost:8020 kylin_tokens

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id B7YqehhCQyZmqCPL6raVv8ncqzLq69EFirjf7Gnfiv7A

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e kylin.substreams.pinax.network:443 graph_out -s 2 -t 0 --production-mode