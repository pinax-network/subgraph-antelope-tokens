.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node http://localhost:8020 eos_tokens
	graph deploy --node=http://localhost:8020 eos_tokens

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id Ce1om4KPxZHwFxhtz2pVuCD4AUiKisrYecHVWsvEW6MU

.PHONY: cache
cache:
	substreams-sink-noop eos.substreams.pinax.network:443 ../../substreams.yaml graph_out 2:

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e eos.substreams.pinax.network:443 graph_out -s 2 -t 0 --production-mode

.PHONY: load
load:
	substreams-sink-graph-load run --graphql-schema ../../schema.graphql tmp/substreams-entities eos.substreams.pinax.network:443 ../../antelope-tokens-v0.3.0.spkg graph_out 398000000

.PHONY: tocsv
tocsv:
	./tocsv.sh
