extern crate goldenfile;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use goldenfile::Mint;

fn setup_file(path: &str, contents: &str) {
    let path = Path::new(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut file = File::create(path).unwrap();
    write!(file, "{}", contents).unwrap();
}

#[test]
fn basic_usage() {
    setup_file("tests/goldenfiles/basic_usage1.txt", "Hello world!");
    setup_file("tests/goldenfiles/basic_usage2.txt", "foobar");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("basic_usage1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("basic_usage2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();
}

#[test]
#[should_panic(expected = "foobar")]
fn positive_diff() {
    setup_file("tests/goldenfiles/positive_diff1.txt", "Hello world!");
    setup_file("tests/goldenfiles/positive_diff2.txt", "foobar");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("positive_diff1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("positive_diff2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "monkeybrains").unwrap();
}

#[test]
fn regeneration() {
    setup_file("tests/goldenfiles/regeneration1.txt", "Junk");
    setup_file("tests/goldenfiles/regeneration2.txt", "More junk");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("regeneration1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("regeneration2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();

    mint.update_goldenfiles();
}

#[test]
#[should_panic(expected = "assertion failed")]
fn external_panic() {
    setup_file("tests/goldenfiles/panic.txt", "old");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("panic.txt").unwrap();

    write!(file1, "new").unwrap();
    assert!(false);
}
