.PHONY: all
all:
	make build
	make pack
	make graph
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run map_transfers -e eth.substreams.pinax.network:443 -s 1000

.PHONY: gui
gui:
	substreams gui map_transfers -e eth.substreams.pinax.network:443 -s -1000

.PHONY: sink
sink:
	substreams gui db_out -e eth.substreams.pinax.network:443 -s -1000
.PHONY: deploy
deploy:
	graph deploy --studio erc-20
