# Cargo Summary

Source : [](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug` directory.
- Use `cargo build --release` to compile it with optimizations. This command will create an executable in   target/release` will make Rust code run faster, but turning on optimization lengthens the time it takes for the program to compile. Only recommended for release builds, not during development.
- Use `cargo run --release` to execute the release binaries. 