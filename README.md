# The Rust Programming Langugae
Introduction to Rust programming, using ["The Rust Programming Language"](https://doc.rust-lang.org/stable/book/foreword.html) by Steve Klabnik and Carol Nichols. 

## Installation on macOS

Follow official installation guide on [Rust](rust-lang.org) official documentation.

1. **Open Terminal**: You can find Terminal in the Applications > Utilities folder or by searching for it using Spotlight.

2. **Install Homebrew** (if you don't have it already):
    ```sh
    $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

3. **Install C Compiler**:
    ```sh
    $ xcode-select --install
    ```

4. **Configure your current shell**:
    ```sh
    source $HOME/.cargo/env
    ```

5. **Verify the installation**:
    ```sh
    rustc --version
    ```
    This command should display the version of Rust that was installed.

Now you have Rust installed on your macOS system and you can start writing Rust programs.

## Data Types
Rust has several built-in data types that can be categorized into scalar and compound types.

### Scalar Types
Scalar types represent a single value. Rust has four primary scalar types:
- **Integers**: Signed and unsigned integers with multiple size options (8-bit, 16-bit, 32-bit, 64-bit, 128-bit, and architecture-specific).
- **Floating-point numbers**: `f32` and `f64` for 32-bit and 64-bit floating-point numbers, respectively.
- **Booleans**: Represented by the `bool` type with two possible values: `true` and `false`.
- **Characters**: Represented by the `char` type, which is a 4-byte Unicode scalar value.

| Length      | Signed | Unsigned |
|-------------|--------|----------|
| 8-bit       | i8     | u8       |
| 16-bit      | i16    | u16      |
| 32-bit      | i32    | u32      |
| 64-bit      | i64    | u64      |
| 128-bit     | i128   | u128     |
| Architecture| isize  | usize    |

### Compound Types
Compound types can group multiple values into one type. Rust has two primary compound types:
- **Tuples**: A fixed-size collection of values of different types. Example: `(i32, f64, char)`.
- **Arrays**: A fixed-size collection of values of the same type. Example: `[i32; 5]` for an array of five 32-bit integers.

## Cross Compiling Rust from Mac to Linux
See https://betterprogramming.pub/cross-compiling-rust-from-mac-to-linux-7fad5a454ab1. 

For windows use 
`brew install mingw-w64`
`cargo build --target x86_64-pc-windows-gnu`

### Known issue
For `./hello_world` error running executable:
```bash
line 1: syntax error near unexpected token `newline`
```

Easier to install rust on the target machine and compile, until resolved.s