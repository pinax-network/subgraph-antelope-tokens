.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: cache
cache:
	substreams-sink-noop eos.substreams.pinax.network:443 substreams.yaml graph_out 2:

.PHONY: gui
gui:
	substreams gui substreams.yaml -e eos.substreams.pinax.network:443 graph_out -s 2 -t 0 --production-mode