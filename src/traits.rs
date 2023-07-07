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
