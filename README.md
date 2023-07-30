# tiny-snake.rs

A very tiny terminal snake game, purely implemented in Rust.

![snake](https://github.com/Rodrigodd/tiny-snake.rs/assets/51273772/1f7c579a-400d-46f5-bf5a-c05f00ae230c)

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

## Building and Running

To build the game binary you only need `rustc` and make:

```shell
$ make snake
```

This creates a binary called `snake` in the current folder, and you can run it
with:

```shell
$ ./snake
```

You can also build and run in a single command:

```shell
$ make run
```

The `Makefile` contains the following commands:

- `snake`: build the main binary.
- `snake.debug`: build the a version of the binary with debug info.
- `size`: print informations about the size of each symbol and section, and total size of the binary.
- `snake.asm`: emmit a clean up version of the assembly emmited by rustc.
- `gdb`: run `snake.debug` on `rust-gdb`.
- `objdump`: produce a clean up disassembly of snake.debug.
- `fmt`: format `snake.rs`
- `clean`: delete all generated files.

## Acknowledgments

- [Kpcyrd's blog post](https://vulns.xyz/2023/03/linux-executable-from-scratch-with-x86_64-unknown-none-rust/)
  about writing a Hello World program using only raw syscalls.
- [Graham King's blog post](https://darkcoding.net/software/a-very-small-rust-binary-indeed/)
  that lists many techniques for reducing the size of a Rust binary, although I
  still didn't use all of them.

## License

This project is licensed under the Unlicense - see the [LICENSE](LICENSE) file
for details.
