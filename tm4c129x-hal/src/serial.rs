//! Serial

use core::fmt;
use core::marker::PhantomData;

use crate::hal::prelude::*;
use crate::hal::serial;
use nb::{self, block};
pub use tm4c129x::{UART0, UART1, UART2, UART3, UART4, UART5, UART6, UART7};
use void::Void;

use crate::gpio::*;
use crate::gpio::{AlternateFunction, OutputMode, AF1};
use crate::sysctl;
use crate::sysctl::Clocks;
use crate::time::Bps;

pub use tm4c_hal::serial::*;

pub use tm4c_hal::{uart_hal_macro, uart_pin_macro};

/// Serial abstraction
pub struct Serial<UART, TX, RX, RTS, CTS> {
    uart: UART,
    tx_pin: TX,
    rx_pin: RX,
    rts_pin: RTS,
    cts_pin: CTS,
    nl_mode: NewlineMode,
}

/// Serial receiver
pub struct Rx<UART, RX, CTS> {
    _uart: PhantomData<UART>,
    pin: RX,
    flow_pin: CTS,
}

/// Serial transmitter
pub struct Tx<UART, TX, RTS> {
    uart: UART,
    pin: TX,
    flow_pin: RTS,
    nl_mode: NewlineMode,
}

uart_pin_macro!(UART0,
    cts: [(gpioh::PH1, AF1), (gpiom::PM4, AF1), (gpiob::PB4, AF1)],
    // dcd: [(gpioh::PH2, AF1), (gpiom::PM5, AF1), (gpiop::PP3, AF2)],
    // dsr: [(gpioh::PH3, AF1), (gpiom::PM6, AF1), (gpiop::PP4, AF2)],
    // dtr: [(gpiop::PP2, AF2)],
    // ri: [(gpiok::PK7, AF1), (gpiom::PM7, AF1)],
    rts: [(gpioh::PH0, AF1), (gpiob::PB5, AF1)],
    rx: [(gpioa::PA0, AF1)],
    tx: [(gpioa::PA1, AF1)],
);

uart_pin_macro!(UART1,
    cts: [(gpiop::PP3, AF1), (gpion::PN1, AF1)],
    // dcd: [(gpioe::PE2, AF1), (gpion::PN2, AF1)],
    // dsr: [(gpioe::PE1, AF1), (gpion::PN3, AF1)],
    // dtr: [(gpioe::PE3, AF1), (gpion::PN3, AF1)],
    // ri: [(gpion::PN5, AF1), (gpion::PE4, AF1)],
    rts: [(gpioe::PE0, AF1), (gpion::PN0, AF1)],
    rx: [(gpiob::PB0, AF1), (gpioq::PQ4, AF1)],
    tx: [(gpiob::PB1, AF1)],
);

uart_pin_macro!(UART2,
    cts: [(gpion::PN3, AF2), (gpiod::PD7, AF1)],
    // dcd: [(gpion::PN2, AF2), (gpiod::PD6, AF1)],
    // dsr: [],
    // dtr: [],
    // ri: [],
    rts: [],
    rx: [(gpioa::PA6, AF1), (gpiod::PD4, AF1)],
    tx: [(gpioa::PA7, AF1), (gpiod::PD5, AF1)],
);

uart_pin_macro!(UART3,
    cts: [],
    // dcd: [],
    // dsr: [],
    // dtr: [],
    // ri: [],
    rts: [],
    rx: [],
    tx: [],
);

uart_pin_macro!(UART4,
    cts: [],
    // dcd: [],
    // dsr: [],
    // dtr: [],
    // ri: [],
    rts: [],
    rx: [],
    tx: [],
);

uart_pin_macro!(UART5,
    cts: [],
    // dcd: [],
    // dsr: [],
    // dtr: [],
    // ri: [],
    rts: [],
    rx: [],
    tx: [],
);

uart_pin_macro!(UART6,
    cts: [],
    // dcd: [],
    // dsr: [],
    // dtr: [],
    // ri: [],
    rts: [],
    rx: [],
    tx: [],
);

uart_pin_macro!(UART7,
    cts: [],
    // dcd: [],
    // dsr: [],
    // dtr: [],
    // ri: [],
    rts: [],
    rx: [],
    tx: [],
);

uart_hal_macro! {
    UART0: (Uart0, uart0),
    UART1: (Uart1, uart1),
    UART2: (Uart2, uart2),
    UART3: (Uart3, uart3),
    UART4: (Uart4, uart4),
    UART5: (Uart5, uart5),
    UART6: (Uart6, uart6),
    UART7: (Uart7, uart7),
}
