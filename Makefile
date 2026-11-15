SHELL := /bin/bash
.SHELLFLAGS := -euo pipefail -c

CARGO        := cargo
BINARY_NAME  := pbl-trpl-106-backend
CLIPPY_FLAGS ?= -- -D warnings

.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@echo "  build            Build debug binary"
	@echo "  release          Build release binary"
	@echo "  build-windows    Cross-compile to Windows .exe"
	@echo "  check            Cargo check"
	@echo "  test             Run tests"
	@echo "  test-verbose     Run tests with nocapture"
	@echo "  fmt              Format code"
	@echo "  fmt-check        Check formatting"
	@echo "  clippy           Run clippy strict"
	@echo "  lint             Run fmt-check and clippy"
	@echo "  run              Run server"
	@echo "  watch            Watch and run server"
	@echo "  serve            Run serve.sh"
	@echo "  smoke            Run smoke tests"
	@echo "  db-up            Start database container"
	@echo "  db-down          Stop database container"
	@echo "  db-reset         Reset database"
	@echo "  ci               Run pipeline"
	@echo "  clean            Clean target"
	@echo "  update           Update dependencies"

.PHONY: all
all: build

.PHONY: build
build:
	$(CARGO) build

.PHONY: release
release:
	$(CARGO) build --release

.PHONY: build-windows
build-windows:
	$(CARGO) xwin build --release --target x86_64-pc-windows-msvc

.PHONY: check
check:
	$(CARGO) check --all-targets

.PHONY: test
test:
	$(CARGO) test --all-targets

.PHONY: test-verbose
test-verbose:
	RUST_BACKTRACE=1 $(CARGO) test --all-targets -- --nocapture

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check

.PHONY: clippy
clippy:
	$(CARGO) clippy --all-targets --all-features $(CLIPPY_FLAGS)

.PHONY: lint
lint: fmt-check clippy

.PHONY: run
run:
	$(CARGO) run

.PHONY: watch
watch:
	$(CARGO) watch -x 'run'

.PHONY: serve
serve:
	./scripts/serve.sh

.PHONY: smoke
smoke:
	./scripts/smoke.sh

.PHONY: db-up
db-up:
	podman-compose up -d

.PHONY: db-down
db-down:
	podman-compose down

.PHONY: db-reset
db-reset:
	./scripts/serve.sh --reset-db

.PHONY: ci
ci: fmt-check clippy test

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: update
update:
	$(CARGO) update
