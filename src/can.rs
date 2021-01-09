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

use crate::gpio::gpiob::{PB12, PB13, PB5, PB6};
use crate::gpio::{
    gpioa::{PA11, PA12},
    gpiob::{PB8, PB9},
    Alternate, Floating, Input, PushPull,
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

impl sealed::Sealed for (PB9<Alternate<PushPull>>, PB8<Input<Floating>>) {}
impl Pins for (PB9<Alternate<PushPull>>, PB8<Input<Floating>>) {
    type Instance = CAN1;

}

impl sealed::Sealed for (PB13<Alternate<PushPull>>, PB12<Input<Floating>>) {}
impl Pins for (PB13<Alternate<PushPull>>, PB12<Input<Floating>>) {
    type Instance = CAN2;

}

/// Interface to the CAN peripheral.
pub struct Can<Instance> {
    _peripheral: Instance,
}

impl<Instance> Can<Instance>
{
    /// Creates a CAN interaface.
    pub fn new(can: Instance, apb: &mut APB1) -> Can<Instance> {
        apb.enr().modify(|_, w| w.can1en().set_bit());
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
