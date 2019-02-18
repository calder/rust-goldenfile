extern crate goldenfile;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use goldenfile::Mint;

fn create_file(path: &str) -> File {
    let path = Path::new(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    File::create(path).unwrap()
}

fn setup_text_file(path: &str, contents: &str) {
    write!(create_file(path), "{}", contents).unwrap();
}

fn setup_binary_file(path: &str, contents: &[u8]) {
    create_file(path).write_all(contents).unwrap();
}

#[test]
fn basic_usage() {
    setup_text_file("tests/goldenfiles/basic_usage1.txt", "Hello world!");
    setup_text_file("tests/goldenfiles/basic_usage2.txt", "foobar");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("basic_usage1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("basic_usage2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();
}

#[test]
fn binary_usage() {
    setup_binary_file("tests/goldenfiles/binary_usage1.bin", b"");
    setup_binary_file("tests/goldenfiles/binary_usage2.bin", b"\x00\x01\x02");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_binary_goldenfile("binary_usage1.bin").unwrap();
    let mut file2 = mint.new_binary_goldenfile("binary_usage2.bin").unwrap();

    file1.write_all(b"").unwrap();
    file2.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
#[should_panic(expected = "File sizes differ: Old file is 2 bytes, new file is 3 bytes")]
fn binary_different_size() {
    setup_binary_file("tests/goldenfiles/binary_different_size.bin", b"\x00\x01");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint
        .new_binary_goldenfile("binary_different_size.bin")
        .unwrap();

    file.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
#[should_panic(expected = "Files differ at byte 3")]
fn binary_different_content() {
    setup_binary_file(
        "tests/goldenfiles/binary_different_content.bin",
        b"\x00\x01\x03",
    );

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint
        .new_binary_goldenfile("binary_different_content.bin")
        .unwrap();

    file.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
#[should_panic(expected = "foobar")]
fn positive_diff() {
    setup_text_file("tests/goldenfiles/positive_diff1.txt", "Hello world!");
    setup_text_file("tests/goldenfiles/positive_diff2.txt", "foobar");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("positive_diff1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("positive_diff2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "monkeybrains").unwrap();
}

#[test]
fn regeneration() {
    setup_text_file("tests/goldenfiles/regeneration1.txt", "Junk");
    setup_text_file("tests/goldenfiles/regeneration2.txt", "More junk");

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
    setup_text_file("tests/goldenfiles/panic.txt", "old");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("panic.txt").unwrap();

    write!(file1, "new").unwrap();
    assert!(false);
}
