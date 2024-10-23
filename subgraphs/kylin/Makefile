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

.PHONY: cache
cache:
	substreams-sink-noop kylin.substreams.pinax.network:443 ../../substreams.yaml graph_out 2:

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e kylin.substreams.pinax.network:443 graph_out -s 2 -t 0 --production-mode

.PHONY: load
load:
	substreams-sink-graph-load run --graphql-schema ../../schema.graphql tmp/substreams-entities kylin.substreams.pinax.network:443 ../../antelope-tokens-v0.3.0.spkg graph_out 370000000 --bundle-size 100000

.PHONY: tocsv
tocsv:
	./tocsv.sh
