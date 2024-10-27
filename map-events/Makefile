.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: cache
cache:
	substreams-sink-noop eos.substreams.pinax.network:443 substreams.yaml map_events 2:

.PHONY: gui
gui:
	substreams gui substreams.yaml -e eos.substreams.pinax.network:443 map_events -s 100000000 --production-mode
