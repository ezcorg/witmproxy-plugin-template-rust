.phony=all clean wasmsign
.DEFAULT_GOAL := all

wasmsign:
	cargo install wasmsign2-cli

key.secret key.public: wasmsign
	wasmsign2 keygen --public-key key.public --secret-key key.secret

all: key.secret key.public
	cargo build --release --target wasm32-wasip2
	wasmsign2 sign -i ./target/wasm32-wasip2/release/{{project-name}}.wasm -o ./target/wasm32-wasip2/release/{{project-name}}.signed.wasm -k key.secret

clean:
	cargo clean --target wasm32-wasip2