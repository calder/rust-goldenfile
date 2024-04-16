# 👑 Rust Goldenfile

[![Documentation](https://docs.rs/goldenfile/badge.svg)](https://docs.rs/goldenfile) [![Latest Version](https://img.shields.io/crates/v/goldenfile.svg)](https://crates.io/crates/goldenfile) [![Build Status](https://github.com/calder/rust-goldenfile/actions/workflows/test.yaml/badge.svg)](https://github.com/calder/rust-goldenfile/actions/workflows/test.yaml?query=branch%3Amain) [![Coverage Status](https://codecov.io/github/calder/rust-goldenfile/branch/main/graph/badge.svg?token=ROF7q6w5no)](https://app.codecov.io/gh/calder/rust-goldenfile)

**Simple goldenfile testing in Rust.**

[Goldenfile](https://softwareengineering.stackexchange.com/questions/358786/what-is-golden-files) tests generate one or more output files as they run. If any files differ from their checked-in "golden" version, the test fails. This ensures that behavioral changes are intentional, explicit, and version controlled.

You can use goldenfiles to test the output of a parser, the order of a graph traversal, the result of a simulation, or anything else that should only change with human review.

## Usage

```rust
use goldenfile::Mint;
use std::io::Write;

let mut mint = Mint::new("tests/goldenfiles");
let mut file1 = mint.new_goldenfile("file1.txt").unwrap();
let mut file2 = mint.new_goldenfile("file2.txt").unwrap();

writeln!(file1, "Hello world!").unwrap();
writeln!(file2, "Foo bar!").unwrap();
```

When the `Mint` goes out of scope, it compares the contents of each file to its checked-in golden version and fails the test if they differ. To update the checked-in versions, run:
```sh
UPDATE_GOLDENFILES=1 cargo test
```

## Contributing

Pull requests are welcome! This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
