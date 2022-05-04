pub struct Flem{
    flem_id: FlemDataId,
}

pub struct FlemCommands;
pub struct FlemConfig;
pub struct FlemStatus;

pub struct FlemDataId {
    name: [char; 30],
    firmware: [char; 30],
    functions: [u32; 10],
}

pub struct FlemGetByte {
    byte: u8,
    info: u16,
}

pub struct FlemAddByte {
    info: u16,
}

pub struct FlemPacket {
    checksum: u16,
    device_cmd: u32,
    device_status: u16,
    length: u16,
    data: [u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize],
    internal_counter: u32,
}

pub type FlemValid = fn(i: FlemInterface, p: &mut FlemPacket);

pub type FlemError = fn(i: FlemInterface, p: &mut FlemPacket, status: u16);

pub struct FlemInterface {
    id: FlemDataId,
    valid_handler: FlemValid,
    error_handler: FlemError,
}

impl FlemStatus {
    pub const FLEM_SUCCESS: u16 = 0;
    pub const FLEM_INFO_PROCESSING: u16 = 1;
    pub const FLEM_INFO_PROCESSED: u16 = 2;

    pub const FLEM_INFO_END_OF_PACKET: u16 = 100;
    pub const FLEM_INFO_CONSTRUCTING_PACKET: u16 = 101;
    pub const FLEM_INFO_DECONSTRUCTING_PACKET: u16 = 102;
    pub const FLEM_INFO_MAX: u16 = 10000;

    pub const FLEM_ERROR_CHECKSUM: u16 = 10001;
    pub const FLEM_ERROR_PACKET_OVERFLOW: u16 = 10002;
    pub const FLEM_ERROR_MAX: u16 = 20000;
}

impl FlemCommands {
    pub const FLEM_ARDUINO_EXAMPLE_COMMAND_OFFSET: u32 = 20000;
    pub const FLEM_SMARTNEB_COMMAND_OFFSET: u32 = 20000 + 10;
    pub const FLEM_SMARTDRAIN_COMMAND_OFFSET: u32 = 20000 + 20;
    
    pub const FLEM_SUCCESS: u32 = 1;
    pub const FLEM_COMMAND_STATUS: u32 = 2;
    pub const FLEM_COMMAND_GET_ARRAY_UINT8: u32 = 3;
    pub const FLEM_COMMAND_GET_ARRAY_INT8: u32 = 4;
    pub const FLEM_COMMAND_GET_ARRAY_UINT16: u32 = 5;
    pub const FLEM_COMMAND_GET_ARRAY_INT16: u32 = 6;
    pub const FLEM_COMMAND_GET_ARRAY_UINT32: u32 = 7;
    pub const FLEM_COMMAND_GET_ARRAY_INT32: u32 = 8;
    pub const FLEM_COMMAND_GET_ARRAY_FLOAT: u32 = 9;
    pub const FLEM_COMMAND_GET_ARRAY_DOUBLE: u32 = 10;
    pub const FLEM_COMMAND_GET_CHAR: u32 = 11;
    pub const FLEM_COMMAND_GET_CUSTOM: u32 = 12;
    pub const FLEM_COMMAND_GET_SCPI: u32 = 13;

    pub const FLEM_SET_OFFSET: u32 = 1e5 as u32;
    pub const FLEM_COMMAND_SET_ARRAY_UINT8: u32 = 3 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_INT8: u32 = 4 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_UINT16: u32 = 5 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_INT16: u32 = 6 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_UINT32: u32 = 7 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_INT32: u32 = 8 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_FLOAT: u32 = 9 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_ARRAY_DOUBLE: u32 = 10 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_CHAR: u32 = 11 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_CUSTOM: u32 = 12 + FlemCommands::FLEM_SET_OFFSET;
    pub const FLEM_COMMAND_SET_SCPI: u32 = 13 + FlemCommands::FLEM_SET_OFFSET;

    
    pub const FLEM_COMMAND_ACQUIRE_ADC: u32 = (FlemCommands::FLEM_ARDUINO_EXAMPLE_COMMAND_OFFSET + 1);
    pub const FLEM_COMMAND_SET_DAC0: u32 = (FlemCommands::FLEM_ARDUINO_EXAMPLE_COMMAND_OFFSET + 2);


    pub const FLEM_COMMAND_GET_FLOW: u32 = (FlemCommands::FLEM_SMARTNEB_COMMAND_OFFSET + 1);

    pub const FLEM_COMMAND_GET_SMART_DRAIN: u32 = (FlemCommands::FLEM_SMARTDRAIN_COMMAND_OFFSET + 1);

}


impl FlemPacket {
    pub const FLEM_MAX_DATA_SIZE: u16 = 100;
    pub const FLEM_HEADER_SIZE: u8 = 10;

    pub fn new() -> Self {
        return Self {
           checksum: 0,
           device_cmd: 0,
           device_status: 0,
           length: 0,
           data: [0u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize],
           internal_counter: 0,
        }
    }

    pub fn getDataArray(&self) -> [u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize] {
        return self.data;
    }

    pub fn add_byte(&mut self, interface: &FlemInterface, byte: &u8) -> FlemAddByte {
        let retval = FlemAddByte {
            info: FlemStatus::FLEM_INFO_CONSTRUCTING_PACKET,
        };



        return retval;
    }

    pub fn get_next_byte(&mut self, interface: &FlemInterface) -> FlemGetByte {
       let retval = FlemGetByte {
           byte: 0,
           info: FlemStatus::FLEM_INFO_DECONSTRUCTING_PACKET,
       };


       return retval;
    }

    pub fn reset(&mut self) {
        self.checksum = 0;
        self.device_cmd = 0;
        self.device_status = 0;
        self.length = 0;
        self.internal_counter = 0;
        self.data = [0u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize];
    }

    pub fn length(&self) -> u16 {
        let mut x: u16 = FlemPacket::FLEM_HEADER_SIZE as u16;
        x += self.length as u16;
        return x;
    }
}
