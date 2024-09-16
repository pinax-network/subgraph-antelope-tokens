.PHONY: all
all:
	make build

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	substreams pack
	substreams info

.PHONY: gui
gui:
	substreams gui -e wax.substreams.pinax.network:443 map_db_ops -s 266683669 --production-mode

.PHONY: run
run:
	substreams run -e wax.substreams.pinax.network:443 map_db_ops -s -1