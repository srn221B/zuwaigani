.PHONY: check ci deploy

check:
	cargo fix --allow-dirty
	cargo clippy
	cargo fmt
	cargo check

ci:
	cargo fmt --check
	cargo clippy --locked --all-targets -- -D warnings
	cargo test --locked

deploy:
	cargo install --path .
