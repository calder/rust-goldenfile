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

fn write_text_file(path: &str, contents: &str) {
    write!(create_file(path), "{}", contents).unwrap();
}

fn write_binary_file(path: &str, contents: &[u8]) {
    create_file(path).write_all(contents).unwrap();
}

#[test]
fn binary_match() {
    write_binary_file("tests/goldenfiles/binary_match1.bin", b"");
    write_binary_file("tests/goldenfiles/binary_match2.bin", b"\x00\x01\x02");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("binary_match1.bin").unwrap();
    let mut file2 = mint.new_goldenfile("binary_match2.bin").unwrap();

    file1.write_all(b"").unwrap();
    file2.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
fn subdir() {
    fs::create_dir_all("tests/goldenfiles/subdir").unwrap();
    write_text_file("tests/goldenfiles/subdir/file1.txt", "File in subdir");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("subdir/file1.txt").unwrap();

    write!(file1, "File in subdir").unwrap();
}

#[test]
#[should_panic(expected = "File sizes differ: Old file is 2 bytes, new file is 3 bytes")]
fn binary_size_diff() {
    write_binary_file("tests/goldenfiles/binary_size_diff.bin", b"\x00\x01");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("binary_size_diff.bin").unwrap();

    file.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
#[should_panic(expected = "Files differ at byte 3")]
fn binary_content_diff() {
    write_binary_file("tests/goldenfiles/binary_content_diff.bin", b"\x00\x01\x03");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("binary_content_diff.bin").unwrap();

    file.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
fn text_match() {
    write_text_file("tests/goldenfiles/match1.txt", "Hello world!");
    write_text_file("tests/goldenfiles/match2.txt", "foobar");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("match1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("match2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();
}

#[test]
#[should_panic(expected = "foobar")]
fn text_diff() {
    write_text_file("tests/goldenfiles/text_diff1.txt", "Hello world!");
    write_text_file("tests/goldenfiles/text_diff2.txt", "foobar");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("text_diff1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("text_diff2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "monkeybrains").unwrap();
}

#[test]
fn regenerate() {
    write_text_file("tests/goldenfiles/regenerate1.txt", "Junk");
    write_text_file("tests/goldenfiles/regenerate2.txt", "More junk");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("regenerate1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("regenerate2.txt").unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();

    mint.update_goldenfiles();
}

#[test]
#[should_panic(expected = "assertion failed")]
fn external_panic() {
    write_text_file("tests/goldenfiles/panic.txt", "old");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("panic.txt").unwrap();

    write!(file1, "new").unwrap();
    assert!(false);
}
