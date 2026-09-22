.PHONY: init test build

init:
	go install github.com/conventionalcommit/commitlint@v0.10.1
	cargo install cargo-watch@8.5.3
	cargo install cargo-nextest@0.9.146 --locked

build:
	cargo build --verbose

test:
	cargo nextest run --all-features --verbose
