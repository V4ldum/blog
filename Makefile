watch:
	cargo leptos watch

setup:
	cargo install cargo-leptos
	rustup target add wasm32-unknown-unknown

build:
	cargo leptos build

run:
	cargo leptos serve

clean:
	rm -r target
