extern crate goldenfile;

use std::io::Write;

use goldenfile::Mint;

#[test]
fn test() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("file1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("file2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "Foo bar!").unwrap();
}
