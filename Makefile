libs := cairo
main: main.c svg_rastor.h
	gcc --std=c99 -O2 main.c -L./target/release -lsvg_rastor $$(pkg-config --cflags --libs ${libs}) -o main

svg_rastor.h: $(wildcard src/*)
	cargo build --release
	touch svg_rastor.h
	cbindgen --crate svg-rastor --output svg_rastor.h --lang c

.PHONEY: clean
clean:
	cargo clean
	rm svg_rastor.h
	rm main
