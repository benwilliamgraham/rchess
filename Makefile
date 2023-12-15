NAME=rchess

all:
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/$(NAME).wasm $(NAME).wasm