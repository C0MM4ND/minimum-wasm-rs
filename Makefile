build-debug:
	cargo build --target wasm32-unknown-unknown
build-release:
	cargo build --target wasm32-unknown-unknown --release
add-target:
	rustup target add wasm32-unknown-unknown