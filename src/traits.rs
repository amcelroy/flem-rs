#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(feature = "std")]
use std::sync::mpsc::{Sender, Receiver};

#[cfg(feature = "std")]
use alloc::{vec::Vec, string::String};

use crate::Packet;

#[derive(Debug, Clone, Copy)]
pub enum DataInterfaceErrors {
    IncorrectBufferLength,
    IncorrectDataLength,
}

pub trait DataInterface<const T: usize>: Sized {
    fn encode(&self, packet: &mut Packet<T>) -> Result<(), DataInterfaceErrors>;
    fn decode(&mut self, packet: &Packet<T>) -> Result<&Self, DataInterfaceErrors>;
}

#[cfg(feature = "std")]
pub trait Host<const T: usize>: Sized {
    type Error;

    fn list_devices(&self) -> Vec<String>;
    fn connect(&mut self, device: &String) -> Result<(), Self::Error>;
    fn disconnect(&mut self) -> Result<(), Self::Error>;
    fn listen(&mut self, rx_sleep_time_ms: u64, tx_sleep_time_ms: u64,) -> (Receiver<Packet<T>>, Sender<Packet<T>>);
    fn unlisten(&mut self) -> Result<(), Self::Error>;
}
