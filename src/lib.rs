//! # Process Listing
//! This crate exposes utilities to deal sequentially with processes and their modules.
//!
//! Examples are found on the function definitions.
//!
//! # Support
//! For now only Windows is supported, but it should be simple enough to port on other operating systems.
//!
//! It's not a priority but pull requests are well accepted.

// We need macros **BEFORE** defining the modules that use them
#[allow(unused)]
macro_rules! trace { ($($x:tt)*) => (
    {
        #[cfg(feature = "log")] {
            log::trace!($($x)*)
        }

        #[cfg(not(feature = "log"))]{
            let _ = format_args!($($x)*);
        }
    }
) }
#[allow(unused)]
macro_rules! debug { ($($x:tt)*) => (
    {
        #[cfg(feature = "log")] {
            log::debug!($($x)*)
        }

        #[cfg(not(feature = "log"))]{
            let _ = format_args!($($x)*);
        }
    }
) }
#[allow(unused)]
macro_rules! info { ($($x:tt)*) => (
    {
        #[cfg(feature = "log")] {
            log::info!($($x)*)
        }

        #[cfg(not(feature = "log"))]{
            let _ = format_args!($($x)*);
        }
    }
) }
#[allow(unused)]
macro_rules! warn { ($($x:tt)*) => (
    {
        #[cfg(feature = "log")] {
            log::warn!($($x)*)
        }

        #[cfg(not(feature = "log"))]{
            let _ = format_args!($($x)*);
        }
    }
) }
#[allow(unused)]
macro_rules! error { ($($x:tt)*) => (
    {
        #[cfg(feature = "log")] {
            log::error!($($x)*)
        }

        #[cfg(not(feature = "log"))]{
            let _ = format_args!($($x)*);
        }
    }
) }

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;
