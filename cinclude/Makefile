default:
	make clean
	cargo build --release --all --lib
	cargo run --release
	mkdir -p lib
	mv target/release/*.a lib/
	mkdir -p inc
	mkdir -p inc/bits
	mv *.h inc/
	mv bits/*.h inc/bits

	cd test/build \
		&& cmake -S .. -B . \
		&& make \
		&& ./stdlib_test \
		|| echo "Error in stdlib_test" \
		&& echo "No errors in stdlib_test" \
		&& cd ../..

clean:
	rm -rf lib/* inc/* test/build/*