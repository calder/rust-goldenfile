extern crate goldenfile;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use goldenfile::{Mint, DifferType};

fn open_file(path: &str) -> File{
let path = Path::new(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    File::create(path).unwrap()
}

fn setup_text_file(path: &str, contents: &str) {
    write!(open_file(path), "{}", contents).unwrap();
}

fn setup_binary_file(path: &str, buffer: &[u8]) {
    open_file(path).write(buffer).unwrap();
}

#[test]
fn basic_usage() {
    setup_text_file("tests/goldenfiles/basic_usage1.txt", "Hello world!");
    setup_text_file("tests/goldenfiles/basic_usage2.txt", "foobar");
    setup_binary_file("tests/goldenfiles/basic_usage3.bin", b"BinaryData\x0f\x0c");
    setup_binary_file("tests/goldenfiles/basic_usage4.bin", b"BinaryData2\xa0\xa0");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("basic_usage1.txt", DifferType::Text).unwrap();
    let mut file2 = mint.new_goldenfile("basic_usage2.txt", DifferType::Text).unwrap();
    let mut file3 = mint.new_goldenfile("basic_usage3.bin", DifferType::Binary).unwrap();
    let mut file4 = mint.new_goldenfile("basic_usage4.bin", DifferType::Binary).unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();
    file3.write(b"BinaryData\x0f\x0c").unwrap();
    file4.write(b"BinaryData2\xa0\xa0").unwrap();
}

#[test]
#[should_panic]
fn positive_diff() {
    setup_text_file("tests/goldenfiles/positive_diff1.txt", "Hello world!");
    setup_text_file("tests/goldenfiles/positive_diff2.txt", "foobar");
    setup_binary_file("tests/goldenfiles/basic_usage3.bin", b"BinaryData\x0f\x0c");
    setup_binary_file("tests/goldenfiles/basic_usage4.bin", b"BinaryData2\xa0\xa0");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("positive_diff1.txt", DifferType::Text).unwrap();
    let mut file2 = mint.new_goldenfile("positive_diff2.txt", DifferType::Text).unwrap();
    let mut file3 = mint.new_goldenfile("positive_diff3.bin", DifferType::Binary).unwrap();
    let mut file4 = mint.new_goldenfile("positive_diff4.bin", DifferType::Binary).unwrap();


    write!(file1, "Hello world!").unwrap();
    write!(file2, "monkeybrains").unwrap();
    file3.write(b"BinaryData\x0f\x0c").unwrap();
    file4.write(b"BinaryData2\xa0\xa0").unwrap();
}

#[test]
#[should_panic]
fn positive_diff_binary() {
    setup_text_file("tests/goldenfiles/positive_diff1.txt", "Hello world!");
    setup_text_file("tests/goldenfiles/positive_diff2.txt", "foobar");
    setup_binary_file("tests/goldenfiles/basic_usage3.bin", b"BinaryData\x0f\x0c");
    setup_binary_file("tests/goldenfiles/basic_usage4.bin", b"BinaryData2\xa0\xa0");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("positive_diff_binary1.txt", DifferType::Text).unwrap();
    let mut file2 = mint.new_goldenfile("positive_diff_binary2.txt", DifferType::Text).unwrap();
    let mut file3 = mint.new_goldenfile("positive_diff_binary3.bin", DifferType::Binary).unwrap();
    let mut file4 = mint.new_goldenfile("positive_diff_binary4.bin", DifferType::Binary).unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();
    file3.write(b"BinaryData\x0f\x0c").unwrap();
    file4.write(b"BinaryData2\xa0\xa0 (not quite the same: \xaa)").unwrap();
}


#[test]
fn regeneration() {
    setup_text_file("tests/goldenfiles/regeneration1.txt", "Junk");
    setup_text_file("tests/goldenfiles/regeneration2.txt", "More junk");
    setup_binary_file("tests/goldenfiles/regeneration3.bin", b"bin \x7c junk");
    setup_binary_file("tests/goldenfiles/regeneration4.bin", b"m\x00re junk");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file1 = mint.new_goldenfile("regeneration1.txt", DifferType::Text).unwrap();
    let mut file2 = mint.new_goldenfile("regeneration2.txt", DifferType::Text).unwrap();
    let mut file3 = mint.new_goldenfile("regeneration3.bin", DifferType::Binary).unwrap();
    let mut file4 = mint.new_goldenfile("regeneration4.bin", DifferType::Binary).unwrap();

    write!(file1, "Hello world!").unwrap();
    write!(file2, "foobar").unwrap();
    file3.write(b"BinaryData\x0f\x0c").unwrap();
    file4.write(b"BinaryData2\xa0\xa0").unwrap();

    mint.update_goldenfiles();
}
