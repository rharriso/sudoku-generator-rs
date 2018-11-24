run: src/*.rs Cargo.toml
	cargo run --release

clean:
	cargo clean

pkg: src/sudoku_generator.rs Cargo.toml
	mkdir -p pkg 
	wasm-pack build --target nodejs
