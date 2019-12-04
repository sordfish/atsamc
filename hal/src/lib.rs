#![no_std]

pub extern crate embedded_hal as hal;

pub use paste;

#[cfg(feature = "samc21n18a")]
pub use atsamd21e18a as target_device;

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

#[cfg(feature="samc21")]
pub mod samc21;
#[cfg(feature="samc21")]
pub use self::samc51::*;

#[cfg(all(feature = "can", feature="samc21"))]
pub use self::samc21::can;

