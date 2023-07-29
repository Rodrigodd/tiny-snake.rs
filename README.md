# tiny-snake.rs

A very tiny terminal snake game, purely implemented in Rust.

https://github.com/Rodrigodd/tiny-snake.rs/assets/51273772/2bbe2954-49c8-49d8-b77c-cab8f0b09df0

## Features

- Optimized binary has only `2760` bytes.
- No dependencies. Not even `libc`.
- Works on x86_64 Linux 3.17+.
- 100% Rust 🦀.
- Panic-free.

## Background

While thinking about implementing the snake in the [bf lang](https://www.google.com/search?client=firefox-b-d&q=brainfuck)
using only [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code),
I decided to build a proof of concept for the snake logic, and ANSI rendering.

I have also seen [this blog post](https://vulns.xyz/2023/03/linux-executable-from-scratch-with-x86_64-unknown-none-rust/)
about writing a Hello World program using raw syscalls, and I figured out that
is all I would need to write the snake game. I also took the opportunity to
implement it in a single file, using `rustc` directly, and avoiding cargo
boilerplate.

In the end, for my delight, I discovered that after building for release and
stripping symbols, of the binary was less than 20 KiB. So I decided invest a
little more in decreasing the binary size.

Changing some parameters parameters for compilation decrease the total size to
about 7KiB, and after making the program panic-free the total size decreased to
less than 3KiB.

There is still room for improvements, but the last change was so dramatic that I
am now satisfied.

## Building

To build the game binary you only need `rustc` and make:

```shell
$ make snake
```

You can also build and run:

```shell
$ make run
```

The `Makefile` contains the following commands:

- `snake`: build the main binary.
- `snake.debug`: build the a version of the binary with debug info.
- `size`: print informations about the size of each symbol and section, and total size of the binary.
- `snake.asm`: emmit a clean up version of the assembly emmited by rustc.
- `gdb`: run `snake.debug` on `rust-gdb`.
- `fmt`: format `snake.rs`
- `clean`: delete all generated files.
