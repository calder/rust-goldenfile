//! Functions for comparing files.

use std::fs::{File, metadata};
use std::io::{Read, BufReader};
use std::path::Path;

use difference;

/// A function that displays a diff and panics if two files to not match.
pub type Differ = Box<Fn(&Path, &Path)>;

/// Compare unicode text files. Print a colored diff and panic on failure.
pub fn text_diff(old: &Path, new: &Path) {
    difference::assert_diff(&read_file(old), &read_file(new), "\n", 0);
}

/// Compare binary files. Print a "Files differ" message and panic on failure.
pub fn binary_diff(old: &Path, new: &Path) {
    let old_metadata = metadata(old)
        .expect(&format!("Error querying for file metadata: {:?}", old));
    let new_metadata = metadata(new)
        .expect(&format!("Error querying for file metadata: {:?}", new));

    if old_metadata.len() != new_metadata.len() {
        panic!("Files differ in size");
    }

    let old_reader = BufReader::new(open_file(old));
    let new_reader = BufReader::new(open_file(new));

    let no_difference = old_reader
        .bytes()
        .zip(new_reader.bytes())
        .all(|(old_result, new_result)| {
            let old_byte = old_result.expect(&format!("Error reading file: {:?}", old));
            let new_byte = new_result.expect(&format!("Error reading file: {:?}", new));

            old_byte == new_byte
        });

    if !no_difference {
        panic!("Files differ in contents");
    }
}

fn read_file(path: &Path) -> String {
    let mut contents = String::new();
    open_file(path)
        .read_to_string(&mut contents)
        .expect(&format!("Error reading file: {:?}", path));

    return contents;
}

fn open_file(path: &Path) -> File {
    File::open(path)
        .expect(&format!("Error opening file: {:?}", path))
}
