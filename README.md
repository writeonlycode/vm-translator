![](https://github.com/writeonlycode/vm-translator/blob/e21a496c509578e3433938602ab4a0607381157d/Screenshot%20from%202024-08-08%2015-15-37.png)

# VM Translator in Rust

This project is an implementation of the VM Translator from the 'Nand to
Tetris' course, written in Rust!

The architecture of the assembler follows a functional approach: mostly
functions doing the heavy work, not relying on objects, and trying to use
mostly immutable variables. However, a few mutable references are used when it
makes everything easier.

The entire implementation is done in just over 300 lines of code!

## Usage

```bash
cargo run -- path/to/filename.vm
```

If `filename.vm` is a valid program, a `filename.asm` file that can be
executed by the Hack computer will be generated!

# References

- [From Nand to Tetris: Building a Modern Computer From First Principles](https://www.nand2tetris.org/)
