# FIXME: the correct egrep expression is '^name *= "[^"]+"$', but what you see below is what works for make. 🤷
PACKAGE = $(shell egrep '^name *= "[^"]+"' Cargo.toml | sed 's/^name *= "\([^"]\+\)"/\1/')

# TODO: add "doc" and "publish" targets

.PHONY: none
none:
	# Please specify a target: build check clean commit format run

# Executes exploratory tests during development
# Note: "cargo run" executes "cargo build" if needed, make does not need to worry about this.
.PHONY: run
run: check
	cargo run -- $(FILE_NAME)

# Lint and syntax checking
.PHONY: check
check: target/debug/$(PACKAGE)
target/debug/$(PACKAGE): src/main.rs
	cargo clippy --workspace --all-features --release -- --deny warnings --forbid clippy::pedantic --forbid clippy::cargo

.PHONY: commit
commit: format
	git commit -S -m '$(MESSAGE)'

.PHONY: format
format:
	cargo fmt --all

# Executes automated tests and generates executable
.PHONY: build
build: build target/release/$(PACKAGE)
target/release/$(PACKAGE): src/main.rs
	make check
	cargo test --workspace
	cargo bench --workspace
	cargo build --release

.PHONY: clean
clean:
	cargo clean
	rm -f Cargo.lock
