.PHONY: test build clean install format run

INSTALL_PREFIX ?= /usr/local

test:
	cargo test -- --test-threads=1

build:
	cargo build --release

clean:
	cargo clean

install:
	cargo install --path . --root $(DESTDIR)$(INSTALL_PREFIX)

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets -- -D warnings

run:
	cargo run
