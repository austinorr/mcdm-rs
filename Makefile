MAKEFLAGS += --silent
.PHONY: install-rust-coverage clean-coverage clean-perf clean build-coverage format lint cov-report cov-show coverage
.DEFAULT_GOAL := all

install-rust-coverage:
	cargo install rustfilt coverage-prepare
	rustup component add llvm-tools-preview

clean-coverage:
	find . -name '*.profdata' -exec rm -fr {} +
	find . -name '*.profraw' -exec rm -fr {} +

clean-perf:
	find . -name '*perf.data*' -exec rm -fr {} +
	find . -name '*flame*.svg' -exec rm -fr {} +

clean: clean-coverage clean-perf
	cargo clean

build-coverage: clean
	RUSTFLAGS="-C instrument-coverage" cargo test

format:
	cargo fmt

lint:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy -- -D warnings -A incomplete_features
	cargo doc

BINARIES = $(eval BINARIES := $$(shell \
	RUSTFLAGS="-C instrument-coverage" \
	cargo test --tests --no-run --message-format=json | \
	jq -r "select(.profile.test == true) | \
	.filenames[]" | \
	grep -v dSYM - | \
	xargs printf "-object %s "))$(BINARIES)

cov-report:
	llvm-profdata merge -sparse default_*.profraw -o rust_coverage.profdata
	llvm-cov report $(BINARIES) --instr-profile=rust_coverage.profdata \
		--ignore-filename-regex=/.cargo/registry \
		--ignore-filename-regex=rustc/.*/library/std/ \
		--ignore-filename-regex=rustc/.*/library/core/

cov-show:
	llvm-profdata merge -sparse default_*.profraw -o rust_coverage.profdata
	rm -fr htmlcov/
	llvm-cov show $(BINARIES) --instr-profile=rust_coverage.profdata \
		--format=html --output-dir=./htmlcov/rust --Xdemangler=rustfilt --show-instantiations=false\
		--ignore-filename-regex=/.cargo/registry \
		--ignore-filename-regex=rustc/.*/library/std/ \
		--ignore-filename-regex=rustc/.*/library/core/

coverage: build-coverage
	$(MAKE) cov-report
	$(MAKE) cov-show
	$(MAKE) clean-coverage

bench:
	cargo build --release
	/usr/bin/time -v ./target/release/promrs

release:
	cargo build -r
