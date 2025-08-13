libs := cairo librsvg-2.0
main: main.c svgtool.h
	gcc --std=c99 -O2 main.c -L./target/release -lsvgtool $$(pkg-config --cflags --libs ${libs}) -o main

svgtool.h: $(wildcard src/*)
	cargo build --release
	touch svgtool.h
	cbindgen --crate svgtool --output svgtool.h --lang c


.PHONEY: run
run: main
	./main $(args)

.PHONEY: clean
clean:
	cargo clean
	rm svgtool.h
	rm main
