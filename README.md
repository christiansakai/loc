# loc

A CLI tool to count lines of code written in Rust.

## Quick Start

```sh
$ cargo run ./my_root_project_path
```

If you have a list of path to be ignored, create a file that contains the list of paths to be ignored, for example, create `ignore_this` file,

```
./target
./Cargo.lock
./Cargo.toml
```

And run it

```sh
$ cargo run --i=./ignore_this ./my_root_project_path
```
