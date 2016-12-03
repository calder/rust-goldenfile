# Rust Goldenfile

**Simple goldenfile testing in Rust.**

Goldenfile tests generate one or more output files as they run. At the end of the test, these output files are compared to the same files produced by previous runs. This ensures that:

  1. Goldenfiles cannot change accidentally.
  2. All changes are explicit and version controlled.

## Usage

```rust
extern crate goldenfile;

use std::io::Write;

use goldenfile::Mint;

#[test]
fn test() {
  let mut mint = Mint::new("tests/goldenfiles");
  let mut file1 = mint.new_goldenfile("file1.txt").unwrap();
  let mut file2 = mint.new_goldenfile("file2.txt").unwrap();

  write!(file1, "Hello ").unwrap();
  write!(file1, "World!").unwrap();
  write!(file2, "Foo").unwrap();
  write!(file2, "Bar").unwrap();

  // When the Mint goes out of scope, it will check the new contents of file1
  // and file2 against their old (golden) contents and fail the test if they
  // differ. The original contents will not be modified unless you run:
  //
  //     env REGENERATE_GOLDENFILES=1 cargo test.
  //
}
```

## Why Goldenfiles?

Goldenfiles often get a bad rap. Used properly, they provide low overhead, insightful tests of a program's operation. "Properly" just means being explicit and selective about what gets written to the goldenfile. You can use them to test the output of a parser, the order of a graph traversal, the results of a simulation, or anything else that shouldn't change without a human's approval.

## Contributing

Feel free to submit pull requests for new content differs or anything else.
