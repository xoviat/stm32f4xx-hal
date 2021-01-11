//! # Controller Area Network (CAN) Interface
//!
//!
//! TX: Alternate Push-Pull Output
//! RX: Input Floating Input
//!
//! ### CAN1
//!
//! | Function | Pins  |
//! |----------|-------|
//! | TX       | PB9   |
//! | RX       | PB8   |
//!
//! ### CAN2
//!
//! | Function | Pins  |
//! |----------|-------|
//! | TX       | PB13  |
//! | RX       | PB12  |

use crate::gpio::{
    gpiob::{PB12, PB13},
    gpiob::{PB8, PB9},
    Alternate, Floating, Input, PushPull, AF9,
};
use crate::pac::CAN1;
use crate::pac::CAN2;
use crate::rcc::APB1;

mod sealed {
    pub trait Sealed {}
}

pub trait Pins: sealed::Sealed {
    type Instance;
}

impl sealed::Sealed for (PB9<Alternate<AF9>>, PB8<Alternate<AF9>>) {}
impl Pins for (PB9<Alternate<AF9>>, PB8<Alternate<AF9>>) {
    type Instance = CAN1;
}

impl sealed::Sealed for (PB13<Alternate<AF9>>, PB12<Alternate<AF9>>) {}
impl Pins for (PB13<Alternate<AF9>>, PB12<Alternate<AF9>>) {
    type Instance = CAN2;
}

/// Interface to the CAN peripheral.
pub struct Can<Instance> {
    _peripheral: Instance,
}

impl<Instance> Can<Instance>
where
    Instance: crate::rcc::Enable<Bus = APB1>,
{
    /// Creates a CAN interaface.
    pub fn new<P>(can: Instance, _pins: P, apb: &mut APB1) -> Can<Instance>
    where
        P: Pins<Instance = Instance>,
    {
        Instance::enable(apb);
        Can { _peripheral: can }
    }
}

unsafe impl bxcan::Instance for Can<CAN1> {
    const REGISTERS: *mut bxcan::RegisterBlock = CAN1::ptr() as *mut _;
}

unsafe impl bxcan::Instance for Can<CAN2> {
    const REGISTERS: *mut bxcan::RegisterBlock = CAN2::ptr() as *mut _;
}

unsafe impl bxcan::FilterOwner for Can<CAN1> {
    const NUM_FILTER_BANKS: u8 = 28;
}

unsafe impl bxcan::MasterInstance for Can<CAN1> {}
