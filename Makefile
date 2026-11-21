.PHONY: check ci frontend-ci frontend-dev deploy

CLI := -p zuwaigani -p zuwaigani-core

check:
	cargo fix --allow-dirty $(CLI)
	cargo clippy $(CLI)
	cargo fmt $(CLI)
	cargo check $(CLI)

ci:
	cargo fmt --check $(CLI)
	cargo clippy --locked $(CLI) --all-targets -- -D warnings
	cargo test --locked $(CLI)

frontend-ci:
	cargo check --locked -p frontend --features ssr
	cargo check --locked -p frontend --lib --features hydrate --target wasm32-unknown-unknown

frontend-dev:
	cd frontend && cargo leptos watch

deploy:
	cargo install --path crates/cli
