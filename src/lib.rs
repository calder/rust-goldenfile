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
//!   write!(file1, "Hello world!").unwrap();
//!   write!(file2, "Foo bar!").unwrap();
//! }
//! ```
//!
//! When the `Mint` goes out of scope, it will compare the contents of each file
//! to its checked-in "golden" version and fail the test if they differ. To
//! update the check-in versions, run:
//! ```sh
//! UPDATE_GOLDENFILES=1 cargo test
//! ```

#![deny(missing_docs)]

pub mod differs;
pub mod mint;

pub use mint::*;
