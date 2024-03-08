MAKEFLAGS += --silent
.PHONY: install-rust-coverage clean-coverage clean-perf clean build-coverage format lint cov-merge cov-report cov-show coverage
.DEFAULT_GOAL := all

install-rust-coverage:
	cargo install rustfilt
	rustup component add llvm-tools-preview

install-cross:
	cargo install cross

clean-coverage:
	find . -wholename '**/*.profdata' -exec rm -fr {} +
	find . -wholename '**/*.profraw' -exec rm -fr {} +
	find . -name '*.lcov' -exec rm -fr {} +

clean-perf:
	find . -name '*perf.data*' -exec rm -fr {} +
	find . -name '*flame*.svg' -exec rm -fr {} +

clean-cargo:
	cargo clean --profile test
	cargo clean --release

clean-so:
	find . -wholename './py-mcdmrs/**/*.so' -exec rm -f {} +

clean: clean-coverage clean-perf

build-coverage: clean
	RUSTFLAGS="-C instrument-coverage" cargo test --tests

build-python: clean
	maturin develop -m py-mcdmrs/Cargo.toml

build-wasm:
	RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
	rustup run nightly-2024-02-22 \
	wasm-pack build --target web -d www/pkg crates/mcdmrs-wasm \
	-- -Z build-std=panic_abort,std

release-python: clean clean-so
	maturin develop -m py-mcdmrs/Cargo.toml --release

coverage-python:
	pytest --cov

format:
	cargo fmt
	
lint:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy
	cargo doc --no-deps

lint-python: lint
	ruff check . --fix
	ruff format . --diff
	pre-commit run --all-files

dBINARIES = $(eval dBINARIES := $$(shell \
	RUSTFLAGS="-C instrument-coverage" \
	cargo test --tests --no-run --message-format=json | \
	jq -r "select(.profile.test == true) | \
	.filenames[]" | \
	grep -v dSYM - | \
	xargs printf "-object %s "))$(dBINARIES)
PACKAGE_NAME=mcdmrs
PACKAGE_BIN=$(shell find . -wholename './target/debug/$(PACKAGE_NAME)' -print -quit)
BINARIES = $(PACKAGE_BIN) $(dBINARIES)

LLVM_IGNORE_EXTERNAL = --ignore-filename-regex=/.cargo/registry \
		--ignore-filename-regex=rustc/.*/library/std/ \
		--ignore-filename-regex=rustc/.*/library/core/

RUSTC_SYSROOT=$(shell rustc --print sysroot)
LLVM_PROFDATA=$(shell find $(RUSTC_SYSROOT) -name llvm-profdata)
LLVM_COV=$(shell find $(RUSTC_SYSROOT) -name llvm-cov)
PROFRAW=$(shell find -wholename **/default*.profraw)

cov-merge:
	$(LLVM_PROFDATA) merge -sparse -o rust_coverage.profdata $(PROFRAW)

cov-report: cov-merge
	$(LLVM_COV) report $(BINARIES) --instr-profile=rust_coverage.profdata \
		$(LLVM_IGNORE_EXTERNAL)

cov-show: cov-merge
	rm -fr htmlcov/
	$(LLVM_COV) show $(BINARIES) --instr-profile=rust_coverage.profdata \
		--format=html --Xdemangler=rustfilt \
		--output-dir=./htmlcov/rust --show-instantiations=false \
		$(LLVM_IGNORE_EXTERNAL)

cov-export: cov-merge
	$(LLVM_COV) export $(BINARIES) --instr-profile=rust_coverage.profdata \
		--format=lcov \
		$(LLVM_IGNORE_EXTERNAL) > $(PACKAGE_NAME).lcov

coverage-ci: build-coverage
	$(MAKE) cov-report
	$(MAKE) cov-export

coverage: coverage-ci
	$(MAKE) cov-show

bench:
	cargo build --release
	/usr/bin/time -v ./target/release/$(PACKAGE_NAME) -a ./examples/data/alternatives_long.csv -c ./examples/data/criteria.csv

release:
	cargo build -r

release-windows:
	cross build --target x86_64-pc-windows-gnu --release
	