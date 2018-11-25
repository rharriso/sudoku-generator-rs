run-js: pkg
	node ./hello.js 100
	
run-native: build-native
	./target/release/runner 100
	
run-native-thread-rng: build-native-thread-rng
	./target/release/runner 100

build-native: src/*.rs Cargo.toml
	cargo +nightly build --release

build-native-thread-rng: src/*.rs Cargo.toml
	cargo +nightly build --features="thread_rng" --release

clean:
	@cargo clean
	@rm -rf pkg

pkg: src/sudoku_generator.rs Cargo.toml
	mkdir -p pkg
	wasm-pack build --target nodejs

