use static_assertions::assert_impl_all;

use goldenfile::Mint;

assert_impl_all!(Mint: Send, Sync);
