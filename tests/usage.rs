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
  //     env REGENERATE_GOLDENFILES=1 cargo test
  //
}
