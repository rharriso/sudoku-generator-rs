run-js: pkg
	node ./hello.js 5
	
run-native: src/*.rs Cargo.toml
	cargo run --release

clean:
	@cargo clean
	@rm -rf pkg

pkg: src/sudoku_generator.rs Cargo.toml
	mkdir -p pkg
	wasm-pack build --target nodejs

benchmark: 
	@for script in generateBoardsRsLoop.js generateBoards.js; do \
		for board_size in 10 100 1000 10000; do \
			echo "$$script $$board_size"; \
			for trial in 1 2 3 4 5; do \
				/usr/bin/time  --output .benchmark-tmp -v node $$script $$board_size; \
				cat .benchmark-tmp | grep "Maximum resident set size (kbytes):"; \
				cat .benchmark-tmp | grep "Elapsed (wall clock) time (h:mm:ss or m:ss):"; \
			done \
		done \
	done
	rm -f .benchmark-tmp
