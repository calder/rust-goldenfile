//! This library provides a simple API for goldenfile testing.
//!
//! Goldenfile tests generate one or more output files as they run. At the end
//! of the test, these output files are compared to the same files produced by
//! previous runs. This ensures that:
//!
//!  1. Goldenfiles cannot change accidentally.
//!  2. All changes are explicit and version controlled.
//!
//! # Example
//!
//! ```rust
//! #[test]
//! fn test() {
//!   let mut mint = Mint::new("tests/goldenfiles");
//!   let mut file1 = mint.new_goldenfile("file1.txt").unwrap();
//!   let mut file2 = mint.new_goldenfile("file2.txt").unwrap();
//!
//!   write!(file1, "Hello ").unwrap();
//!   write!(file1, "World!").unwrap();
//!   write!(file2, "Foo").unwrap();
//!   write!(file2, "Bar").unwrap();
//!
//!   // When the Mint goes out of scope, it will check the new contents of file1
//!   // and file2 against their old (golden) contents and fail the test if they
//!   // differ. The original contents will not be modified unless you run:
//!   //
//!   //     env REGENERATE_GOLDENFILES=1 cargo test.
//!   //
//! }
//! ```

#![deny(missing_docs)]

extern crate difference;
extern crate tempdir;

pub mod differs;
pub mod mint;

pub use mint::*;
