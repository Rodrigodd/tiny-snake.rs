# tiny-snake.rs

A very tiny terminal snake game, purely implemented in Rust.

https://github.com/Rodrigodd/tiny-snake.rs/assets/51273772/2bbe2954-49c8-49d8-b77c-cab8f0b09df0

## Building

To build the game binary you only need rustc and make:

```shell
$ make snake
```

You can also build and run:

```shell
$ make run
```

In general the `Makefile` contains the following commands:

- `snake`: build the main binary.
- `snake.debug`: build the a version of the binary with debug info.
- `size`: print informations about the size of each symbol, section and total size of the binary.
- `snake.asm`: emmit a clean up version of the assembly emmited by rustc.
- `gdb`: run `snake.debug` on `rust-gdb`.
- `fmt`: format `snake.rs`
- `clean`: delete all generated files.

