use std::env;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use tempdir::TempDir;

use differs::*;

pub struct Mint {
    path: PathBuf,
    tempdir: TempDir,
    files: Vec<(PathBuf, Differ)>,
}

impl Mint {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let tempdir = TempDir::new("rust-goldenfiles").unwrap();
        let mint = Mint {
            path: path.as_ref().to_path_buf(),
            files: vec![],
            tempdir: tempdir,
        };
        fs::create_dir_all(mint.goldenfile_path())
            .expect(&format!("Failed to create goldenfile directory {:?}",
                             mint.goldenfile_path()));
        return mint;
    }

    pub fn new_goldenfile<P: AsRef<Path>>(&mut self, path: P) -> Result<File> {
        if path.as_ref().is_absolute() {
            return Err(Error::new(ErrorKind::InvalidInput, "Path must be relative."));
        }

        let abs_path = self.tempdir.path().to_path_buf().join(path.as_ref());
        let maybe_file = File::create(abs_path.clone());
        if maybe_file.is_ok() {
            // TODO: Use other differs for different file extensions.
            let differ = Box::new(text_diff);
            self.files.push((path.as_ref().to_path_buf(), differ));
        }
        maybe_file
    }

    pub fn check_goldenfiles(&self) {
        for &(ref file, ref differ) in &self.files {
            let old = self.goldenfile_path().join(&file);
            let new = self.tempdir.path().join(&file);

            println!("\nGoldenfile diff for {:?}:", file.to_str().unwrap());
            println!("To regenerate the goldenfile, run");
            println!("    env REGENERATE_GOLDENFILES=1 cargo test");
            println!("------------------------------------------------------------");
            differ(&old, &new);
            println!("<NO DIFFERENCE>");
        }
    }

    pub fn update_goldenfiles(&self) {
        for &(ref file, _) in &self.files {
            let old = self.goldenfile_path().join(&file);
            let new = self.tempdir.path().join(&file);

            println!("Updating {:?}.", file.to_str().unwrap());
            fs::copy(&new, &old).expect(&format!("Error copying {:?} to {:?}.", &new, &old));
        }
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
        if regen_var.is_ok() && regen_var.unwrap() == "1" {
            self.update_goldenfiles();
        } else {
            self.check_goldenfiles();
        }
    }
}
