.PHONY: check ci capabilities daisyui frontend-ci frontend-dev deploy

CLI := -p zuwaigani -p zuwaigani-core
FRONTEND := -p frontend
DAISYUI_VERSION := v5.8.2

check:
	cargo fix --allow-dirty $(CLI)
	cargo clippy $(CLI)
	cargo fmt $(CLI) $(FRONTEND)
	cargo check $(CLI)
	cargo clippy $(FRONTEND) --features ssr
	cargo clippy $(FRONTEND) --lib --features hydrate --target wasm32-unknown-unknown

ci:
	cargo fmt --check $(CLI)
	cargo clippy --locked $(CLI) --all-targets -- -D warnings
	cargo test --locked $(CLI)

capabilities:
	cargo run -q -p zuwaigani -- platforms --json > frontend/data/capabilities.json

daisyui:
	curl -sSL -o frontend/style/daisyui.mjs https://github.com/saadeghi/daisyui/releases/download/$(DAISYUI_VERSION)/daisyui.mjs

frontend-ci:
	cargo fmt --check $(FRONTEND)
	cargo clippy --locked $(FRONTEND) --features ssr -- -D warnings
	cargo clippy --locked $(FRONTEND) --lib --features hydrate --target wasm32-unknown-unknown -- -D warnings

frontend-dev:
	cd frontend && cargo leptos watch

deploy:
	cargo install --path crates/cli
