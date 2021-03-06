#![no_std]

pub extern crate embedded_hal as hal;

pub use paste;

#[cfg(feature = "samc21e18a")]
pub use atsamc21e18a as target_device;

#[cfg(feature = "samc21g18a")]
pub use atsamc21g18a as target_device;

#[cfg(feature = "samc21j18a")]
pub use atsamc21j18a as target_device;

#[cfg(feature = "samc21n18a")]
pub use atsamc21n18a as target_device;

#[cfg(feature = "use_rtt")]
pub use jlink_rtt;

#[cfg(feature = "use_rtt")]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut out = $crate::jlink_rtt::NonBlockingOutput::new();
            writeln!(out, $($arg)*).ok();
        }
    };
}

#[cfg(all(not(feature = "use_rtt"), not(feature = "use_uart_debug")))]
#[macro_export]
macro_rules! dbgprint {
    ($($arg:tt)*) => {{}};
}

#[macro_use]
pub mod common;
pub use self::common::*;
