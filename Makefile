build:
	mkdir -p bin
	cargo build --release
	cp target/release/tac0 bin/

clean:
	rm -rf bin/
	rm -r target/