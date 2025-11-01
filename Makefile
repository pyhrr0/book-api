.PHONY: run
run:
	cargo run --bin book_api serve

.PHONY: build
build:
	cargo build --release --bin book_api

.PHONY: clean
clean:
	cargo clean

# cargo install cargo-watch
.PHONY: watch
watch:
	cargo watch -x 'run --color=always --bin book_api serve 2>&1 | less'

.PHONY: test
test:
	cargo test

.PHONY: container
CRI:=$(shell type -p podman || echo docker)
container:
	$(CRI) build -f Dockerfile -t book_api:latest .

.PHONY: check
check: udeps clippy

## cargo install cargo-udeps
.PHONY: udeps
udeps:
	cargo +nightly udeps

.PHONY: clippy
clippy:
	cargo clippy
