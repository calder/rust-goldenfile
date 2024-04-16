extern crate goldenfile;

use std::fs;
use std::io::Write;

use goldenfile::Mint;

#[test]
fn binary_match() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("binary_match1.bin").unwrap();
    let mut file2 = mint.new_goldenfile("binary_match2.bin").unwrap();

    file1.write_all(b"").unwrap();
    file2.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
fn subdir() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("subdir/file1.txt").unwrap();

    writeln!(file1, "File in subdir").unwrap();
}

#[test]
#[should_panic(expected = "File sizes differ: Old file is 2 bytes, new file is 3 bytes")]
fn binary_size_diff() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("binary_size_diff.bin").unwrap();

    file.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
#[should_panic(expected = "Files differ at byte 3")]
fn binary_content_diff() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("binary_content_diff.bin").unwrap();

    file.write_all(b"\x00\x01\x02").unwrap();
}

#[test]
fn text_match() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("match1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("match2.txt").unwrap();

    writeln!(file1, "Hello world!").unwrap();
    writeln!(file2, "foobar").unwrap();
}

#[test]
#[should_panic(expected = "foobar")]
fn text_diff() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("text_diff1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("text_diff2.txt").unwrap();

    writeln!(file1, "Hello world!").unwrap();
    writeln!(file2, "monkeybrains").unwrap();
}

#[test]
#[should_panic(expected = "Path must be relative")]
fn absolute_path() {
    let mut mint = Mint::new("tests/goldenfiles");
    mint.new_goldenfile("/bar").unwrap();
}

#[test]
#[should_panic(expected = "assertion failed")]
fn external_panic() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("panic.txt").unwrap();

    writeln!(file1, "new").unwrap();
    assert!(false);
}

#[test]
fn update() {
    fs::remove_file("tests/goldenfiles/update_env1.txt").unwrap();
    fs::remove_file("tests/goldenfiles/update_env2.txt").unwrap();

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("update_env1.txt").unwrap();
    let mut file2 = mint.new_goldenfile("update_env2.txt").unwrap();

    writeln!(file1, "Hello world!").unwrap();
    writeln!(file2, "foobar").unwrap();

    mint.update_goldenfiles()
}

#[test]
fn nonempty() {
    let mut mint = Mint::new_nonempty("tests/goldenfiles");
    let mut file = mint.new_goldenfile("nonempty.txt").unwrap();
    mint.new_goldenfile("empty.txt").unwrap();

    writeln!(file, "Some content").unwrap();
}
