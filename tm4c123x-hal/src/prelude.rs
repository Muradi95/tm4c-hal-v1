//! Prelude

#[rustfmt::skip]
pub use crate::{
    gpio::GpioExt as _,
    hal::digital::*,
    hal::delay::DelayNs,
    sysctl::SysctlExt,
    time::U32Ext,
};
