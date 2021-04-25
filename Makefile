COLOR ?= auto # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)
CARGO_TEST = cargo test --test server_tests test --all-features --manifest-path ./crates/multisig-service/Cargo.toml -- --nocapture --show-output --test-threads=1


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
	@$(CARGO) run -p pos-service

test: build
	@$(CARGO_TEST)

update:
	@$(CARGO) update
