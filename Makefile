test: build lint
	cargo test --workspace

build:
	cargo build --workspace

lint:
	cargo clippy --workspace -- -D warnings

.PHONY: build lint test
