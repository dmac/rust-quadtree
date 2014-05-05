CRATE_ROOT = src/quadtree.rs
RLIB_FILE = $(shell rustc --crate-type=rlib --crate-file-name $(CRATE_ROOT))
RLIB = lib/$(RLIB_FILE)

SFML_RLIB_FILE = $(shell (rustc --crate-type=rlib --crate-file-name lib/rust-sfml/src/lib.rs 2> /dev/null) \
                   || (echo "sfml-placeholder.rlib"))
SFML_RLIB = lib/$(SFML_RLIB_FILE)

default: lib

all: lib demo doc

lib: $(RLIB)

demo: bin/demo

doc: $(CRATE_ROOT)
	rustdoc $(CRATE_ROOT)

$(RLIB): $(CRATE_ROOT)
	mkdir -p lib
	rustc --crate-type=rlib -o $@ $(CRATE_ROOT)

bin/demo: deps $(RLIB) src/demo.rs
	mkdir -p bin
	rustc -L lib -o $@ src/demo.rs

deps: $(SFML_RLIB)

$(SFML_RLIB): lib/rust-sfml
	cd lib/rust-sfml && make rsfml && cd .. && cp rust-sfml/lib/*.rlib .

lib/rust-sfml:
	mkdir -p lib
	git clone https://github.com/JeremyLetang/rust-sfml $@

.PHONY: test
test: $(CRATE_ROOT)
	mkdir -p bin
	rustc --test -o bin/test $(CRATE_ROOT)
	bin/test

.PHONY: clean
clean:
	rm -fr bin doc $(RLIB)

.PHONY: clean-deps
clean-deps:
	rm -fr $(SFML_RLIB) lib/rust-sfml

.PHONY: clean-all
clean-all: clean clean-deps
	rmdir lib
