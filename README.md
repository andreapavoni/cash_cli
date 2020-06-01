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
- [ ] Basic features + ideas/iterations
  - [x] record withdraw
  - [x] record deposit
  - [x] list records (by month and/or category)
    - [ ] set month and/or year from-to ranges (maybe -M and -Y?)
  - [x] report stats (by month and/or category)
    - [ ] set month and/or year from-to ranges (maybe -M and -Y?)
  - [x] import from CSV
    - [x] provide CSV files for demos data
    - [x] templates to import operations: sets same date day and current month + year
      - [ ] set month and/or year on the CLI options
  - [ ] export to CSV
- [ ] (re)write proper tests
- [ ] remove all those `unwrap()`s and add better error handling

## MAYBE

- [ ] remote server for storage/sync (aka cloud)
  - users and API keys
  - no web ui OR barebones equivalent of a CLI ?
    - same features as API and hypothetichal web UI like the CLI app
