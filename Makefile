# TODO: add "doc" and "publish" targets.

.PHONY: none
none:
	# Please specify a target: build check clean commit format run

# Executes exploratory tests during development.
# Note: "cargo run" executes "cargo build" iff needed, make does not need to worry about this.
.PHONY: run
run: check
	cargo run -- $(FILE_NAME)

# Lint and syntax checking (see https://doc.rust-lang.org/stable/clippy/index.html).
# TODO I want to run cargo clippy for each crate separately, and only as needed.
.PHONY: check
check:
	cargo clippy --workspace --all-features --release -- --forbid clippy::all --forbid clippy::pedantic --forbid clippy::cargo

# Format code before committing, so we never get diff issues over formatting alone.
# Also, signs all commits.
.PHONY: commit
commit: format
	git commit -S -m '$(MESSAGE)'

.PHONY: format
format:
	cargo fmt --all

# Executes automated tests and, if they pass, generates executable.
# TODO I want to run tests for each crate separately, and only as needed.
.PHONY: build
build:
	make check
	cargo test --workspace
	cargo bench --workspace
	cargo build --release

.PHONY: clean
clean:
	cargo clean
	rm -f Cargo.lock
