extern crate goldenfile;

use std::io::Write;

use goldenfile::{Mint, DifferType};

#[test]
fn test() {
  let mut mint = Mint::new("tests/goldenfiles");
  let mut file1 = mint.new_goldenfile("file1.txt", DifferType::Text).unwrap();
  let mut file2 = mint.new_goldenfile("file2.bin", DifferType::Binary).unwrap();

  write!(file1, "Hello ").unwrap();
  write!(file1, "World!").unwrap();
  file2.write(b"Binary data: \x7f\x4c").unwrap();
  file2.write(b"More binary data: \x9a\x23").unwrap();

  // When the Mint goes out of scope, it will check the new contents of file1
  // and file2 against their old (golden) contents and fail the test if they
  // differ. The original contents will not be modified unless you run:
  //
  //     env REGENERATE_GOLDENFILES=1 cargo test
  //
}
