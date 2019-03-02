# Rust FFI Example for Java & Python

Example project for sharing logic between Python & Java using rust FFI.

## Why?

Python and Java are two popular languages, sometimes we need them to share a common
logic, for example calculating some formula or running domain specific algorithm.

FFI has been around for a while but writing C/C++ code considered unsafe, which most of the 
time is unacceptable on memory managed languages like Python & Java.

Rust comes to the picture with a safe(r) memory management model,
with comparable speed to C/C++ that compiles to native shared libraries so we can enjoy both worlds and 
make Python & Java friends again (not really :).


## Getting Started

### Dependencies

* rustc & cargo nightly (nightly for PyO3)
* Python3
* Java (1.8+ preferred)
* make (for automating commands)

*tested on ubuntu-18.04 x64*


run these commands:

```bash
$ make build
$ make test
```

now you can just copy the shared libraries from `dist/` directory


## Project Structure

we got 3 packages on our Cargo workspace:

* **rs-divider** - shared library implementation in rust
* **rs-divider-java** - java bindings for rs-divider
* **rs-divider-py** - python bindings for rs-divider


## Project Structure:

```
.
├── Cargo.lock
├── Cargo.toml          # our workspace, definition of the 3 packages
├── dist/               # contains java & python shared libraries (.so files)
├── Makefile            # for automating build and test tasks
├── rs-divider          # our shared rust lib
│   ├── Cargo.toml
│   └── src
│       └── lib.rs      # our shared rust lib source code
├── rs-divider-java     # java binding to rs-divider
│   ├── Cargo.toml
│   ├── RsDivider.h     # auto generated jni header file. contains the function signature for our lib
│   ├── RsDivider.java  # example main for using our rsdivider lib from java
│   └── src
│       └── lib.rs      # src of the wrapper between java and rust using the `jni` crate
└── rs-divider-py       # python binding to rs-divider
    ├── Cargo.toml
    ├── rsdivider.py    # example main for using rsdivider lib from python
    └── src
        └── lib.rs      # code using `pyo3` crate for wrapping rs-divider as python shared lib
```


## License

MIT & APACHE dual license
