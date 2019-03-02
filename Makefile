build:
	cargo build --release
	rm -rf dist/
	mkdir dist
	cp ./target/release/librsdividerjava.so dist/
	cp ./target/release/librsdivider_py.so dist/rsdivider_py.so

test-rust: build
	cargo test

test-python: build
	rm -f target/release/rsdivider_py.so
	PYTHONPATH='./dist/' python rs-divider-py/rsdivider.py

test-java: build
	rm -f rs-divider-java/RsDivider.h
	cd rs-divider-java/ && \
		javah RsDivider && \
		javac RsDivider.java && java -Djava.library.path=../dist/ RsDivider

test: test-rust test-python test-java
