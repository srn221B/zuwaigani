
check:
	cargo fix --allow-dirty
	cargo clippy
	cargo fmt
	cargo check

deploy:
	cargo install --path .
