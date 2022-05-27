export OUTD=dist
export BIN:=mdals

default: setup build run

setup:
	mkdir -p $(OUTD)
	rm -rf $(OUTD)/*

build:
	cargo build
	cp -r ./target/debug/mdals ./$(OUTD)/

run:
	./$(OUTD)/$(BIN)
	

%.json, %.js:
	pnpm install
	pnpm run start

%.rs:
	cargo build --release
	cargo run
