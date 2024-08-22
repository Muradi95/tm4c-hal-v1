//! Prelude

#[rustfmt::skip]
pub use crate::{
    gpio::GpioExt as _,
    hal::digital::*,
    sysctl::SysctlExt,
    time::U32Ext,
};
