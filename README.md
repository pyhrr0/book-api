# Book API
A simple REST API demonstrating full CRUD operations, with health-check and metric endpoints.

## How to run
```bash
$ set -a
$ source .env.development
$ ./target/release/book_api serve
```
## Development
The following tools are required to be able to develop on this project:

- [Rust development environment](https://rust-lang.github.io/rustup/installation/index.html)
- [cargo-watch](https://github.com/watchexec/cargo-watch)
- [cargo-udeps](https://github.com/est31/cargo-udeps)

A Makefile has been created, which contains the following targets:
  - run: build & start the API server.
  - build: build a shippable binary.
  - clean: remove generated build artifacts.
  - watch: build & start the API server, and re[start|build] it in case a change has been made to the source code.
  - test: run integration & unit tests.
  - container: build a container, and tag it as `book_api:latest`.
  - check: verify if required tools are available.
  - udeps: find unused dependencies.
  - clippy: run linter.
