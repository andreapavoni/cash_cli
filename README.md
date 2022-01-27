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
target/debug/cash -h
```

### Simple usage

```
# manually add records
target/debug/cash record withdraw -a "100" --category "Lifestyle" --label "Club" -d "2020-05-27" -D "Dancing"

# import demo data (may 2020)
target/debug/cash import data.demo.csv

# import demo template for recurrent records
target/debug/cash import template.demo.csv --template

# List records for may 2020
target/debug/cash list -m 5 -y 2020

+----------------+--------------+------------+-------------+---------------+
| Date           | Category     | Label      | Description |               |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-02     | Bills        | Services   | iCloud      |     -1,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-04     | Bills        | Services   | Netflix     |    -16,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-08     | Bills        | Internet   | May 2020    |    -60,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-08     | Bills        | Mobile     | May 2020    |     -8,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-10     | Earnings     | Paycheck   | April 2020  |  5 000,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-13     | Fundamentals | Car        | Diesel      |    -20,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-20     | Extra        | Bank       | Mortgage    |   -600,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-23     | Lifestyle    | Restaurant | Ice cream   |     -1,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-23     | Optional     | Shopping   | Covid mask  |     -8,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-25     | Bills        | Energy     | May 2020    |   -100,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-25     | Fundamentals | Car        | Check       |   -250,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-27     | Bills        | Services   | Spotify     |    -10,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-28     | Fundamentals | Car        | Diesel      |    -20,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-31     | Optional     | Shopping   | Tshirt      |    -25,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-31     | Family       | Kids       | Doll        |    -10,00 €   |
+----------------+--------------+------------+-------------+---------------+
| 2020-05-31     | Family       | Kids       | Robot       |    -10,00 €   |
+----------------+--------------+------------+-------------+---------------+
+----------------+--------------+------------+-------------+---------------+
| Total earnings |              |            |             |  5 000,00 €   |
+----------------+--------------+------------+-------------+---------------+
| Total expenses |              |            |             | -1 139,00 €   |
+----------------+--------------+------------+-------------+---------------+
| Total          |              |            |             |  3 861,00 €   |
+----------------+--------------+------------+-------------+---------------+


# report for may 2020 by grouped by categories
target/debug/cash report -m 5 -y 2020

+--------------+--------------+
| Fundamentals |  -290,00 €   |
+--------------+--------------+
| Extra        |  -600,00 €   |
+--------------+--------------+
| Optional     |   -33,00 €   |
+--------------+--------------+
| Lifestyle    |    -1,00 €   |
+--------------+--------------+
| Family       |   -20,00 €   |
+--------------+--------------+
| Bills        |  -195,00 €   |
+--------------+--------------+
| Earnings     | 5 000,00 €   |
+--------------+--------------+
+--------------+--------------+
| Total        | 3 861,00 €   |
+--------------+--------------+

# report for may 2020 grouped by labels in category "Bills"
target/debug/cash report -m 5 -y 2020 -c Bills

+----------+-------------+
| Mobile   |   -8,00 €   |
+----------+-------------+
| Internet |  -60,00 €   |
+----------+-------------+
| Energy   | -100,00 €   |
+----------+-------------+
| Services |  -27,00 €   |
+----------+-------------+
+----------+-------------+
| Total    | -195,00 €   |
+----------+-------------+
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
