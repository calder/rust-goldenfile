//! Goldenfile tests generate one or more output files as they run. If any files
//! differ from their checked-in "golden" version, the test fails. This ensures
//! that behavioral changes are intentional, explicit, and version controlled.
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
//!   write!(file1, "Hello world!").unwrap();
//!   write!(file2, "Foo bar!").unwrap();
//! }
//! ```
//!
//! When the `Mint` goes out of scope, it compares the contents of each file
//! to its checked-in golden version and fails the test if they differ. To
//! update the checked-in versions, run:
//! ```sh
//! UPDATE_GOLDENFILES=1 cargo test
//! ```

#![deny(missing_docs)]

pub mod differs;
pub mod mint;

pub use mint::*;
