# TODO: the correct egrep expression is '^name *= "[^"]+"$', but what you see below is what works for make. 🤷
PACKAGE = $(shell egrep '^name *= "[^"]+"' Cargo.toml | sed 's/^name *= "\([^"]\+\)"/\1/')

BENCH   = cargo bench --workspace
BUILD   = cargo build
FORMAT  = cargo fmt --all
LINT    = cargo clippy --workspace --all-features --release -- --deny warnings --forbid clippy::pedantic --forbid clippy::cargo
TEST    = cargo test --workspace

# TODO: add "doc" and "publish" targets
# TODO: make targets "format," "lint" and "test" not .PHONY, so we don't run them unless we have to (output a file after each)
# TODO: create a "bench" target. Make it not .PHONY

.PHONY: none
none:
	# Please specify a target: build clean format lint release run test

.PHONY: release
release: build target/release/$(PACKAGE)
target/release/$(PACKAGE): target/debug/$(PACKAGE)
	$(BENCH)
	$(BUILD) --release

.PHONY: run
run: build
	cargo run -- $(FILE_NAME)

.PHONY: build
build: target/debug/$(PACKAGE)
target/debug/$(PACKAGE): src/main.rs
	make test
	$(BUILD)

.PHONY: test
test:
	$(LINT)
	$(TEST)

.PHONY: lint
lint:
	$(LINT)

.PHONY: clean
clean:
	cargo clean
	rm -f Cargo.lock

.PHONY: format
format:
	$(FORMAT)

.PHONY: commit
commit: format
	git commit -S -m '$(MESSAGE)'
