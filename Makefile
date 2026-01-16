.PHONY: fmt clippy build test check

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

build:
	cargo build

test:
	cargo test

check: fmt clippy build test
