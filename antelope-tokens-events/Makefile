.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info
	substreams graph

.PHONY: protogen
protogen:
	substreams protogen

.PHONY: cache
cache:
	substreams-sink-noop kylin.substreams.pinax.network:443 substreams.yaml map_events 2:

.PHONY: gui
gui:
	substreams gui substreams.yaml -e kylin.substreams.pinax.network:443 map_events -s 4452494

.PHONY: parquet
parquet:
	substreams-sink-files run eos.substreams.pinax.network:443 substreams.yaml map_events '.' 2: --encoder parquet

.PHONY: schema
schema:
	substreams-sink-files tools parquet schema substreams.yaml map_events