# CSV Analyzer

A command-line tool for analyzing CSV files, written in Rust.

## Features

- View file structure and data types
- Statistical operations (average, min, max)
- Display first/last rows
- Find unique values with counts
- Colored terminal output

## Installation

```bash
git clone https://github.com/arondubovszky/rust-csv-analyzer
cd rust-csv-analyzer
cargo build --release
```

## Usage

```bash
# Basic file info
./target/release/rust-csv-analyzer data.csv columns
./target/release/rust-csv-analyzer data.csv rows

# View data
./target/release/csva data.csv head 5
./target/release/csva data.csv tail 10
./target/release/csva data.csv show

# Statistics
./target/release/csva data.csv avg salary
./target/release/csva data.csv min age
./target/release/csva data.csv max price

# Analysis
./target/release/csva data.csv unique department

# Help
./target/release/csva help
```

## Example

```
$ ./target/release/csva employees.csv avg salary
Average: 62500.00

$ ./target/release/csva employees.csv unique department
Column 'department' has 3 unique elements:
Engineering : 15
Marketing : 8
Sales : 12
```

## Requirements

- Rust 1.0 or later
- CSV files with comma-separated values

Built as a learning project to explore Rust programming.
