use std::env;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use tempdir::TempDir;

use differs;

pub struct Mint {
    path: PathBuf,
    tempdir: TempDir,
    files: Vec<PathBuf>,
    differ: Box<Fn(&Path, &Path)>,
}

impl Mint {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        // TODO: Use a different differ for certain file extensions.
        Self::new_with_differ(path, Box::new(differs::text_diff))
    }

    pub fn new_with_differ<P: AsRef<Path>>(path: P, differ: Box<Fn(&Path, &Path)>) -> Self {
        let tempdir = TempDir::new("rust-goldenfiles").unwrap();
        Mint {
            path: path.as_ref().to_path_buf(),
            files: vec![],
            tempdir: tempdir,
            differ: differ,
        }
    }

    pub fn new_goldenfile<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        if path.as_ref().is_absolute() {
            return Err(Error::new(ErrorKind::InvalidInput, "Path must be relative."));
        }

        let abs_path = self.tempdir.path().to_path_buf().join(path.as_ref());
        let maybe_file = File::create(abs_path.clone());
        if maybe_file.is_ok() {
            self.files.push(path.as_ref().to_path_buf());
        }
        maybe_file
    }

    fn goldenfile_path(&self) -> PathBuf {
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(self.path.clone())
    }
}

impl Drop for Mint {
    fn drop(&mut self) {
        let regen_var = env::var("REGENERATE_GOLDENFILES");
        let regen = regen_var.is_ok() && regen_var.unwrap() == "1";

        for file in &self.files {
            let old = self.goldenfile_path().join(&file);
            let new = self.tempdir.path().join(&file);

            if regen {
                println!("Updating {:?}.", file.to_str().unwrap());
                fs::copy(&new, &old).expect(&format!("Error copying {:?} to {:?}.", &new, &old));
            } else {
                println!("\nGoldenfile diff for {:?}:", file.to_str().unwrap());
                println!("To regenerate the goldenfile, run");
                println!("    env REGENERATE_GOLDENFILES=1 cargo test");
                println!("------------------------------------------------------------");
                (self.differ)(&old, &new);
                println!("<NO DIFFERENCE>");
            }
        }
    }
}
