//! Used to create goldenfiles.

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::thread;

use tempfile::TempDir;

use crate::differs::*;

/// A Mint creates goldenfiles.
///
/// When a Mint goes out of scope, it will do one of two things depending on the
/// value of the `REGENERATE_GOLDENFILES` environment variable:
///
///   1. If `REGENERATE_GOLDENFILES!=1`, it will check the new goldenfile
///      contents against their old contents, and panic if they differ.
///   2. If `REGENERATE_GOLDENFILES=1`, it will replace the old goldenfile
///      contents with the newly written contents.
pub struct Mint {
    path: PathBuf,
    tempdir: TempDir,
    files: Vec<(PathBuf, Differ)>,
}

impl Mint {
    /// Create a new goldenfile Mint.
    ///
    /// All goldenfiles will be created in the Mint's directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let tempdir = TempDir::new().unwrap();
        let mint = Mint {
            path: path.as_ref().to_path_buf(),
            files: vec![],
            tempdir: tempdir,
        };
        fs::create_dir_all(&mint.path).expect(&format!(
            "Failed to create goldenfile directory {:?}",
            mint.path
        ));
        return mint;
    }

    /// Create a new goldenfile using a differ inferred from the file extension.
    ///
    /// The returned File is a temporary file, not the goldenfile itself.
    pub fn new_goldenfile<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        self.new_goldenfile_with_differ(&path, get_differ_for_path(&path))
    }

    /// Create a new goldenfile with the specified diff function.
    ///
    /// The returned File is a temporary file, not the goldenfile itself.
    pub fn new_goldenfile_with_differ<P: AsRef<Path>>(
        &mut self,
        path: P,
        differ: Differ,
    ) -> Result<File> {
        if path.as_ref().is_absolute() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Path must be relative.",
            ));
        }

        let abs_path = self.tempdir.path().to_path_buf().join(path.as_ref());
        let maybe_file = File::create(abs_path.clone());
        if maybe_file.is_ok() {
            self.files.push((path.as_ref().to_path_buf(), differ));
        }
        maybe_file
    }

    /// Check new goldenfile contents against old, and panic if they differ.
    ///
    /// Called automatically when a Mint goes out of scope and
    /// `REGENERATE_GOLDENFILES!=1`.
    pub fn check_goldenfiles(&self) {
        for &(ref file, ref differ) in &self.files {
            let old = self.path.join(&file);
            let new = self.tempdir.path().join(&file);

            println!("\nGoldenfile diff for {:?}:", file.to_str().unwrap());
            println!("To regenerate the goldenfile, run");
            println!("    env REGENERATE_GOLDENFILES=1 cargo test");
            println!("------------------------------------------------------------");
            differ(&old, &new);
            println!("<NO DIFFERENCE>");
        }
    }

    /// Overwrite old goldenfile contents with their new contents.
    ///
    /// Called automatically when a Mint goes out of scope and
    /// `REGENERATE_GOLDENFILES=1`.
    pub fn update_goldenfiles(&self) {
        for &(ref file, _) in &self.files {
            let old = self.path.join(&file);
            let new = self.tempdir.path().join(&file);

            println!("Updating {:?}.", file.to_str().unwrap());
            fs::copy(&new, &old).expect(&format!("Error copying {:?} to {:?}.", &new, &old));
        }
    }
}

/// Get the diff function to use for a given file path.
pub fn get_differ_for_path<P: AsRef<Path>>(_path: P) -> Differ {
    match _path.as_ref().extension() {
        Some(os_str) => match os_str.to_str() {
            Some("bin") => Box::new(binary_diff),
            Some("exe") => Box::new(binary_diff),
            Some("gz") => Box::new(binary_diff),
            Some("tar") => Box::new(binary_diff),
            Some("zip") => Box::new(binary_diff),
            _ => Box::new(text_diff),
        },
        _ => Box::new(text_diff),
    }
}

impl Drop for Mint {
    /// Called when the mint goes out of scope to check or update goldenfiles.
    fn drop(&mut self) {
        if thread::panicking() {
            return;
        }
        let regen_var = env::var("REGENERATE_GOLDENFILES");
        if regen_var.is_ok() && regen_var.unwrap() == "1" {
            self.update_goldenfiles();
        } else {
            self.check_goldenfiles();
        }
    }
}
