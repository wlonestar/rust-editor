# rust-editor - a "rust" text editor

## Background

I learned and tried to reproduce this small command line project 
in order to learn the rust programming language.

Original link: [@otukof/build-your-text-editor-with-rust](https://medium.com/@otukof/build-your-text-editor-with-rust-678a463f968b)

This project has been slightly modified on the basis of referring to that blog.

## Usage

### just run to see 

```shell
$ cargo run
# or open to edit a file
$ cargo run <the file to open>
```

### build

Please refer to [document](https://rust-lang.github.io/rustup/cross-compilation.html) for the necessary toolchain.

```shell
$ cargo build --release --target x86_64-pc-windows-msvc
# or
$ cargo build --release --target x86_64-unknown-linux-musl
```
