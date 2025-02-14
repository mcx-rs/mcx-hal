//! System Controller

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "mcxa")] {
        mod mrcc;
        pub use mrcc::*;
    }
}
