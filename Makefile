SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc
UPX := $(shell command -v upx 2> /dev/null)

.PHONY: all
all:
	cargo build --release 
	strip target/release/gist-rs
ifdef UPX
		upx target/release/gist-rs
endif
	cargo install --path . 

.PHONY: build
build:
	cargo build --release 
	strip target/release/gist-rs
	upx target/release/gist-rs

.PHONY: build-linux
build-linux:
	cargo build --target x86_64-unknown-linux-musl --release
	strip target/x86_64-unknown-linux-musl/release/gist-rs
	upx target/x86_64-unknown-linux-musl/release/gist-rs

.PHONY: build-win
build-win:
	RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" cargo build --target x86_64-pc-windows-gnu --release
	strip target/x86_64-pc-windows-gnu/release/gist-rs.exe
	upx target/x86_64-pc-windows-gnu/release/gist-rs.exe

.PHONY: build-mac
build-mac:
	cargo build --target x86_64-apple-darwin --release
	strip target/x86_64-apple-darwin/release/gist-rs
	upx target/x86_64-apple-darwin/release/gist-rs

.PHONY: run
run:
ifndef ARGS
	@echo Run "make run" with ARGS set to pass arguments…
endif
	cargo run --release -- $(ARGS)

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean
