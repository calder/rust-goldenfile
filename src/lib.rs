//! This library provides a simple API for goldenfile testing.
//!
//! Goldenfile tests generate one or more output files as they run. At the end
//! of the test, the generated files are compared to checked-in "golden" files
//! produced by previous runs. This ensures that all changes to goldenfiles are
//! intentional, explicit, and version controlled.
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
//!   // and file2 against their version controlled "golden" contents and fail the
//!   // test if they differ.
//!   //
//!   // To update the goldenfiles themselves, run:
//!   //
//!   //     REGENERATE_GOLDENFILES=1 cargo test
//!   //
//! }
//! ```

#![deny(missing_docs)]

pub mod differs;
pub mod mint;

pub use mint::*;
