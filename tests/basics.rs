extern crate goldenfile;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use goldenfile::Mint;

fn assert_singlethreaded() {
    let threads = env::var("RUST_TEST_THREADS");
    if !threads.is_ok() || threads.unwrap() != "1" {
        print!("\nERROR: This test sets environment variables and can't be ");
        println!("run concurrently with other tests. Rerun with:");
        println!("    env RUST_TEST_THREADS=1 cargo test\n");
        panic!("Cannot run test in parallel mode.");
    }
}

fn setup_file(path: &str, contents: &str) {
    let path = env::current_exe()
        .unwrap().parent()
        .unwrap().parent()
        .unwrap().parent()
        .unwrap()
        .join(Path::new(path));
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
#[should_panic]
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
    assert_singlethreaded();

    {
        env::set_var("REGENERATE_GOLDENFILES", "1");

        let mut mint = Mint::new("tests/goldenfiles");
        let mut file1 = mint.new_goldenfile("regeneration1.txt").unwrap();
        let mut file2 = mint.new_goldenfile("regeneration2.txt").unwrap();

        write!(file1, "Hello world!").unwrap();
        write!(file2, "foobar").unwrap();
    }

    {
        env::remove_var("REGENERATE_GOLDENFILES");

        let mut mint = Mint::new("tests/goldenfiles");
        let mut file1 = mint.new_goldenfile("regeneration1.txt").unwrap();
        let mut file2 = mint.new_goldenfile("regeneration2.txt").unwrap();

        write!(file1, "Hello world!").unwrap();
        write!(file2, "foobar").unwrap();
    }
}
