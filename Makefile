.PHONY: init build test fmt clippy

init:
	go install github.com/conventionalcommit/commitlint@v0.10.1
	cargo install cargo-watch@8.5.3
	cargo install cargo-nextest@0.9.146 --locked

build:
	cargo build --release --verbose

test:
	cargo nextest run --all-features --verbose

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings
