# Cash

Simple, experimental CLI app to track personal finances. It's starting as a way to learn
and train myself with Rust.

## Dependencies

You need SQLite installed.

## Quick setup

```
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run
cargo build
cargo test
cargo run --bin cash -- -h
```

It's still _heavily work in progress_, but it works. See TODO below.

## TODO

- [ ] write a better README with some fancy example output
- [ ] Basic features
  - [x] record withdraw
  - [x] record deposit
  - [x] list records (by month and/or category)
  - [x] report stats (by month and/or category)
  - [x] import from CSV
    - [ ] provide a CSV file for demo
  - [ ] export to CSV
- [ ] (re)write proper tests
- [ ] remove all those `unwrap()`s and add better error handling
