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
	substreams-sink-noop wax.substreams.pinax.network:443 substreams.yaml graph_out 2: -H "X-Sf-Substreams-Parallel-Jobs: 250"

.PHONY: gui
gui:
	substreams gui substreams.yaml -e wax.substreams.pinax.network:443 graph_out -s -20000 -t 0