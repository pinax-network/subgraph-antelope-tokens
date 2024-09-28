.PHONY: all
all:
	make build

.PHONY: build
build:
	graph build

.PHONY: deploy
deploy:
	graph build
	graph create --node http://localhost:8020 wax_tokens
	graph deploy --node=http://localhost:8020 wax_tokens

.PHONY: publish
publish:
	graph build
	graph publish --subgraph-id 6Tt5mHVNDyAo3KbsYMUeemmzs2381vXUquXw75EnG9cW

.PHONY: gui
gui:
	substreams gui ../../substreams.yaml -e wax.substreams.pinax.network:443 graph_out -s 2 -t 0 --production-mode