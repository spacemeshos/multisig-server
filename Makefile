COLOR ?= auto # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
CARGO_TEST = cargo test --bin multisig-service server::tests --no-fail-fast --all-features --color=always --manifest-path ./crates/server/Cargo.toml -- --nocapture --show-output --test-threads=1

.PHONY: all bench build check clean doc install publish run test update format

all: build

format:
	@$(CARGO) fmt

bench:
	@$(CARGO) bench

build: format
	 @$(CARGO) build --color=always --all --all-targets

check:
	@$(CARGO) check

clean:
	@$(CARGO) clean

doc:
	@$(CARGO) doc

install: build
	@$(CARGO) install

publish:
	@$(CARGO) publish

run: build
	@$(CARGO) run -p server

test: build
	@$(CARGO_TEST)

update:
	@$(CARGO) update
