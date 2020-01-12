//! Process Listing.
//!
//! This crate exposes a `for_each_processes` function to deal sequentially with every process open in the operating system.

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