watch:
	cargo leptos watch

setup:
	cargo install cargo-leptos
	rustup target add wasm32-unknown-unknown

build:
	cargo leptos build

run:
	cargo leptos serve

deploy:
	sudo docker build -t blog .
	sudo docker run -it -p 3000:3000 --mount type=bind,source="$(shell pwd)/posts",target=/app/posts blog

clean:
	rm -r target
