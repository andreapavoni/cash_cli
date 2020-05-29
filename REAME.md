# Cash

Simple, experimental CLI app to track personal finances. It's starting as a way to learn
and train myself with Rust.

## Setup

```
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run
cargo build
cargo test
cargo run --bin cash -- -h
```

It actually isn't interactive at all, if you run it, it will print some string to show you
how it works.

## Roadm... ehm... Ideas?

- CLI options for:

  - adding a new operation (deposit, withdraw)
  - show the list of operations actually done
  - show stats for all categories
  - show stats for all labels of a given category
  - specify a different config file path

- Configuration

  - read a file, fallback on defaults
  - where to load/store stuff

- Storage

  - save/load data
