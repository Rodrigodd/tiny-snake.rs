# Some args were copied from: https://darkcoding.net/software/a-very-small-rust-binary-indeed/

ARGS = \
	-C opt-level=z \
	-C debuginfo=2 \
	-C panic=abort \
	-C strip=symbols \
	-C link-args=-N \
	-C relocation-model=static \
	--target x86_64-unknown-none \
	--edition=2021
	# these flags didn't decrease the size \
	-C link-args=-n \
	-C link-args=--no-dynamic-linker \
	-C link-args=--no-pie \
	-C link-args=--no-eh-frame-hdr \
	-C link-args=--build-id=none \
	-C link-args=--no-rosegment \
	-C link-args=-O2 \

snake: snake.rs
	rustc $(ARGS) -o snake snake.rs

snake.debug: snake.rs
	rustc $(ARGS) -C strip=none -o snake.debug snake.rs

size: snake snake.debug
	nm -S --size-sort -t d snake.debug | rustfilt
	readelf -SW snake | sed -n '/^Key to Flags:/q;p' | grep -A 9999999 "Section Headers:"
	ls -l snake

run: snake
	./snake

gdb: snake.debug
	rust-gdb --args snake.debug

snake.asm: snake.rs
	# emit asm with intel syntax, and demangles with rustfilt
	rustc -o snake.asm snake.rs $(ARGS) --emit asm -C llvm-args=-x86-asm-syntax=intel && cat snake.asm | sed '/^\s*\./d' | rustfilt | sponge snake.asm

fmt:
	rustfmt snake.rs --edition=2021

clean:
	rm -f snake snake.asm snake.debug
