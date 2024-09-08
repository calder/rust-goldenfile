//! Functions for comparing files.

use std::fs;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

use similar_asserts;

/// A function that displays a diff and panics if two files to not match.
pub type Differ = Box<dyn Fn(&Path, &Path)>;

/// Compare unicode text files. Print a colored diff and panic on failure.
pub fn text_diff(old: &Path, new: &Path) {
    similar_asserts::assert_eq!(
        &fs::read_to_string(old).unwrap_or("".to_string()),
        &fs::read_to_string(new).unwrap_or("".to_string()),
        "{}",
        old.display(),
    );
}

/// Panic if binary files differ with some basic information about where they
/// differ.
pub fn binary_diff(old: &Path, new: &Path) {
    let old_len = file_len(old);
    let new_len = file_len(new);
    if old_len != new_len {
        panic!(
            "File sizes differ: Old file is {} bytes, new file is {} bytes",
            old_len, new_len
        );
    }

    let first_difference = file_byte_iter(old)
        .zip(file_byte_iter(new))
        .position(|(old_byte, new_byte)| old_byte != new_byte);

    if let Some(position) = first_difference {
        panic!("{}: Files differ at byte {}", old.display(), position + 1);
    }
}

fn open_file(path: &Path) -> fs::File {
    check_io(fs::File::open(path), "opening file", path)
}

fn file_byte_iter(path: &Path) -> impl Iterator<Item = u8> + '_ {
    BufReader::new(open_file(path))
        .bytes()
        .map(move |b| check_io(b, "reading file", path))
}

fn file_len(path: &Path) -> u64 {
    check_io(fs::metadata(path), "getting file length", path).len()
}

fn check_io<T>(x: Result<T, io::Error>, message: &str, path: &Path) -> T {
    x.unwrap_or_else(|err| panic!("Error {} {:?}: {:?}", message, path, err))
}
