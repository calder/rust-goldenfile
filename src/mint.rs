//! Used to create goldenfiles.

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use tempfile::TempDir;
use yansi::Paint;

use crate::differs::*;

/// A Mint creates goldenfiles.
///
/// When a Mint goes out of scope, it will do one of two things depending on the
/// value of the `UPDATE_GOLDENFILES` environment variable:
///
///   1. If `UPDATE_GOLDENFILES!=1`, it will check the new goldenfile
///      contents against their old contents, and panic if they differ.
///   2. If `UPDATE_GOLDENFILES=1`, it will replace the old goldenfile
///      contents with the newly written contents.
pub struct Mint {
    state: Arc<Mutex<MintState>>,
}

/// Internal Mint state.
struct MintState {
    path: PathBuf,
    tempdir: TempDir,
    files: Vec<(PathBuf, Differ)>,
    create_empty: bool,
}

impl Mint {
    /// Create a new goldenfile Mint.
    fn new_internal<P: AsRef<Path>>(path: P, create_empty: bool) -> Self {
        let tempdir = TempDir::new().unwrap();
        let path = path.as_ref().to_path_buf();
        let files = vec![];

        fs::create_dir_all(&path).unwrap_or_else(|err| {
            panic!(
                "Failed to create goldenfile directory {:?}: {:?}",
                path, err
            )
        });

        Mint {
            state: Arc::new(Mutex::new(MintState {
                path,
                tempdir,
                files,
                create_empty,
            })),
        }
    }

    /// Create a new goldenfile Mint.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self::new_internal(path, true)
    }

    /// Create a new goldenfile Mint. Goldenfiles will only be created when non-empty.
    pub fn new_nonempty<P: AsRef<Path>>(path: P) -> Self {
        Self::new_internal(path, false)
    }

    /// Create a new goldenfile using a differ inferred from the file extension.
    ///
    /// The returned File is a temporary file, not the goldenfile itself.
    pub fn new_goldenfile<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        self.state.lock().unwrap().new_goldenfile(path)
    }

    /// Create a new goldenfile with the specified diff function.
    ///
    /// The returned File is a temporary file, not the goldenfile itself.
    pub fn new_goldenfile_with_differ<P: AsRef<Path>>(
        &mut self,
        path: P,
        differ: Differ,
    ) -> Result<File> {
        self.state
            .lock()
            .unwrap()
            .new_goldenfile_with_differ(path, differ)
    }

    /// Check new goldenfile contents against old, and panic if they differ.
    ///
    /// Called automatically when a Mint goes out of scope and
    /// `UPDATE_GOLDENFILES!=1`.
    pub fn check_goldenfiles(&self) {
        self.state.lock().unwrap().check_goldenfiles();
    }

    /// Overwrite old goldenfile contents with their new contents.
    ///
    /// Called automatically when a Mint goes out of scope and
    /// `UPDATE_GOLDENFILES=1`.
    pub fn update_goldenfiles(&self) {
        self.state.lock().unwrap().update_goldenfiles();
    }

    /// Register a new goldenfile using a differ inferred from the file extension.
    ///
    /// The returned PathBuf references a temporary file, not the goldenfile itself.
    pub fn new_goldenpath<P: AsRef<Path>>(&mut self, path: P) -> Result<PathBuf> {
        self.state.lock().unwrap().new_goldenpath(path)
    }

    /// Register a new goldenfile with the specified diff function.
    ///
    /// The returned PathBuf references a temporary file, not the goldenfile itself.
    pub fn new_goldenpath_with_differ<P: AsRef<Path>>(
        &mut self,
        path: P,
        differ: Differ,
    ) -> Result<PathBuf> {
        self.state
            .lock()
            .unwrap()
            .new_goldenpath_with_differ(path, differ)
    }
}

impl MintState {
    pub fn new_goldenfile<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        self.new_goldenfile_with_differ(&path, get_differ_for_path(&path))
    }

    pub fn new_goldenfile_with_differ<P: AsRef<Path>>(
        &mut self,
        path: P,
        differ: Differ,
    ) -> Result<File> {
        let abs_path = self.new_goldenpath_with_differ(path, differ)?;
        if let Some(abs_parent) = abs_path.parent()
            && abs_parent != self.tempdir.path()
        {
            fs::create_dir_all(abs_parent).unwrap_or_else(|err| {
                panic!(
                    "Failed to create temporary subdirectory {:?}: {:?}",
                    abs_parent, err
                )
            });
        }

        let maybe_file = File::create(abs_path);
        if maybe_file.is_err() {
            self.files.pop();
        }

        maybe_file
    }

    pub fn check_goldenfiles(&self) {
        for (file, differ) in &self.files {
            let old = self.path.join(file);
            let new = self.tempdir.path().join(file);
            defer_on_unwind! {
                eprintln!("note: run with `UPDATE_GOLDENFILES=1` to update goldenfiles");
                eprintln!(
                    "{}: goldenfile changed: {}",
                    "error".bold().red(),
                    file.to_str().unwrap()
                );
            }
            differ(&old, &new);
        }
    }

    pub fn update_goldenfiles(&self) {
        for (file, _) in &self.files {
            let old = self.path.join(file);
            let new = self.tempdir.path().join(file);

            let empty = File::open(&new).unwrap().metadata().unwrap().len() == 0;
            if self.create_empty || !empty {
                println!("Updating {:?}.", file.to_str().unwrap());
                fs::copy(&new, &old).unwrap_or_else(|err| {
                    panic!("Error copying {:?} to {:?}: {:?}", &new, &old, err)
                });
            } else if old.exists() {
                std::fs::remove_file(&old).unwrap();
            }
        }
    }

    pub fn new_goldenpath<P: AsRef<Path>>(&mut self, path: P) -> Result<PathBuf> {
        self.new_goldenpath_with_differ(&path, get_differ_for_path(&path))
    }

    pub fn new_goldenpath_with_differ<P: AsRef<Path>>(
        &mut self,
        path: P,
        differ: Differ,
    ) -> Result<PathBuf> {
        if !path.as_ref().is_relative() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Path must be relative.",
            ));
        }

        let abs_path = self.tempdir.path().to_path_buf().join(path.as_ref());
        self.files.push((path.as_ref().to_path_buf(), differ));

        Ok(abs_path)
    }
}

/// Get the diff function to use for a given file path.
pub fn get_differ_for_path<P: AsRef<Path>>(_path: P) -> Differ {
    match _path.as_ref().extension() {
        Some(os_str) => match os_str.to_str() {
            Some("bin") => Box::new(binary_diff),
            Some("exe") => Box::new(binary_diff),
            Some("gz") => Box::new(binary_diff),
            Some("pcap") => Box::new(binary_diff),
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
        // For backwards compatibility with 1.4 and below.
        let legacy_var = env::var("REGENERATE_GOLDENFILES");
        let update_var = env::var("UPDATE_GOLDENFILES");
        if (legacy_var.is_ok() && legacy_var.unwrap() == "1")
            || (update_var.is_ok() && update_var.unwrap() == "1")
        {
            self.update_goldenfiles();
        } else {
            self.check_goldenfiles();
        }
    }
}
