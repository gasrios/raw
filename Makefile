# TODO: add "doc" and "publish" targets.

.PHONY: none
none:
	# Please specify a target: build check clean commit format run

# Executes exploratory tests during development. Format code iff all tests pass.
# Note: "cargo run" executes "cargo build" iff needed, make does not need to worry about this.
.PHONY: run
run: check
	cargo run -- $(FILE_NAME)
	make format

# Executes automated tests and, if they pass, generates executable.
# TODO Run tests for each crate separately, and only as needed.
.PHONY: build
build: check
	cargo build --release

# Ensures code passes tests before committing and signs commit.
.PHONY: commit
commit: check
	git commit -S -m '$(MESSAGE)'

# Lint and syntax checking (see https://doc.rust-lang.org/stable/clippy/index.html).
# TODO Run cargo clippy for each crate separately, and only as needed.
.PHONY: check
check:
	cargo clippy --workspace --all-features --release -- --deny warnings --forbid clippy::all --forbid clippy::pedantic --forbid clippy::cargo
	cargo test --workspace
	cargo bench --workspace

.PHONY: format
format:
	cargo fmt --all

.PHONY: clean
clean:
	cargo clean
	rm -f Cargo.lock

.PHONY: todo
todo:
	egrep -n 'TODO|FIXME' $(shell find -type f -name '*.rs') | sed 's/\(^[^:]\+:[0-9]\+:\) *\(.*\)/\1\t\2/'
