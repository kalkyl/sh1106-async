#![no_std]

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

extern crate embedded_hal as hal;

pub mod builder;
mod command;
pub mod displayrotation;
pub mod displaysize;
pub mod interface;
pub mod mode;
pub mod prelude;
pub mod properties;

pub use crate::builder::Builder;
