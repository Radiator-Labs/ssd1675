#![no_std]

extern crate embedded_hal as hal;

#[cfg(test)]
#[macro_use]
extern crate std;

mod color;
mod command;
mod config;
mod display;
mod graphics;
mod interface;

pub use color::Color;
pub use config::Builder;
pub use display::{Dimensions, Display, Rotation};
pub use graphics::GraphicDisplay;
pub use interface::DisplayInterface;
pub use interface::Interface;
