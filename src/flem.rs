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

impl FlemDataId {
    pub fn new(name: &str, firmware: &str, functions: &[u32]) -> Result<Self, &'static str> {
        let mut id = FlemDataId {
            name: ['\0'; 30],
            firmware: ['\0'; 30],
            functions: [0; 10],
        };

        if name.len() > 30 {
            return Err("Error: name should be 30 chars or less");
        }else{
            for a in 0..name.len() {
                let x = a as usize;
                id.name[x] = name.as_bytes()[x] as char;
            }
        }

        if firmware.len() > 30 {
            return Err("Error: firmware should be 30 chars or less");
        }else{
            for a in 0..firmware.len() {
                let x = a as usize;
                id.firmware[x] == firmware.as_bytes()[x] as char;
            }
        }

        if functions.len() > 10 {
            return Err("Error: functions should be 10 elements or less");
        }else{
            for a in 0..functions.len() {
               id.functions[a] = functions[a]; 
            }
        }

        Ok(id)
    }
    
}

pub struct FlemGetByte {
    byte: u8,
    info: u16,
}

pub struct FlemAddByte {
    info: u16,
}

#[repr(C, packed)]
pub struct FlemPacket {
    checksum: u16,
    device_cmd: u32,
    device_status: u16,
    length: u16,
    data: [u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize],
    internal_counter: u32,
    length_counter: usize,
}

pub type FlemValid = fn(i: &FlemInterface, p: &mut FlemPacket);

pub type FlemError = fn(i: &FlemInterface, p: &mut FlemPacket, status: u16);

pub struct FlemInterface {
    pub id: FlemDataId,
    pub valid_handler: FlemValid,
    pub error_handler: FlemError,
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

    pub const crc16_tab: [u16; 256] = [
        0x0000, 0xc0c1, 0xc181, 0x0140, 0xc301, 0x03c0, 0x0280, 0xc241,
        0xc601, 0x06c0, 0x0780, 0xc741, 0x0500, 0xc5c1, 0xc481, 0x0440,
        0xcc01, 0x0cc0, 0x0d80, 0xcd41, 0x0f00, 0xcfc1, 0xce81, 0x0e40,
        0x0a00, 0xcac1, 0xcb81, 0x0b40, 0xc901, 0x09c0, 0x0880, 0xc841,
        0xd801, 0x18c0, 0x1980, 0xd941, 0x1b00, 0xdbc1, 0xda81, 0x1a40,
        0x1e00, 0xdec1, 0xdf81, 0x1f40, 0xdd01, 0x1dc0, 0x1c80, 0xdc41,
        0x1400, 0xd4c1, 0xd581, 0x1540, 0xd701, 0x17c0, 0x1680, 0xd641,
        0xd201, 0x12c0, 0x1380, 0xd341, 0x1100, 0xd1c1, 0xd081, 0x1040,
        0xf001, 0x30c0, 0x3180, 0xf141, 0x3300, 0xf3c1, 0xf281, 0x3240,
        0x3600, 0xf6c1, 0xf781, 0x3740, 0xf501, 0x35c0, 0x3480, 0xf441,
        0x3c00, 0xfcc1, 0xfd81, 0x3d40, 0xff01, 0x3fc0, 0x3e80, 0xfe41,
        0xfa01, 0x3ac0, 0x3b80, 0xfb41, 0x3900, 0xf9c1, 0xf881, 0x3840,
        0x2800, 0xe8c1, 0xe981, 0x2940, 0xeb01, 0x2bc0, 0x2a80, 0xea41,
        0xee01, 0x2ec0, 0x2f80, 0xef41, 0x2d00, 0xedc1, 0xec81, 0x2c40,
        0xe401, 0x24c0, 0x2580, 0xe541, 0x2700, 0xe7c1, 0xe681, 0x2640,
        0x2200, 0xe2c1, 0xe381, 0x2340, 0xe101, 0x21c0, 0x2080, 0xe041,
        0xa001, 0x60c0, 0x6180, 0xa141, 0x6300, 0xa3c1, 0xa281, 0x6240,
        0x6600, 0xa6c1, 0xa781, 0x6740, 0xa501, 0x65c0, 0x6480, 0xa441,
        0x6c00, 0xacc1, 0xad81, 0x6d40, 0xaf01, 0x6fc0, 0x6e80, 0xae41,
        0xaa01, 0x6ac0, 0x6b80, 0xab41, 0x6900, 0xa9c1, 0xa881, 0x6840,
        0x7800, 0xb8c1, 0xb981, 0x7940, 0xbb01, 0x7bc0, 0x7a80, 0xba41,
        0xbe01, 0x7ec0, 0x7f80, 0xbf41, 0x7d00, 0xbdc1, 0xbc81, 0x7c40,
        0xb401, 0x74c0, 0x7580, 0xb541, 0x7700, 0xb7c1, 0xb681, 0x7640,
        0x7200, 0xb2c1, 0xb381, 0x7340, 0xb101, 0x71c0, 0x7080, 0xb041,
        0x5000, 0x90c1, 0x9181, 0x5140, 0x9301, 0x53c0, 0x5280, 0x9241,
        0x9601, 0x56c0, 0x5780, 0x9741, 0x5500, 0x95c1, 0x9481, 0x5440,
        0x9c01, 0x5cc0, 0x5d80, 0x9d41, 0x5f00, 0x9fc1, 0x9e81, 0x5e40,
        0x5a00, 0x9ac1, 0x9b81, 0x5b40, 0x9901, 0x59c0, 0x5880, 0x9841,
        0x8801, 0x48c0, 0x4980, 0x8941, 0x4b00, 0x8bc1, 0x8a81, 0x4a40,
        0x4e00, 0x8ec1, 0x8f81, 0x4f40, 0x8d01, 0x4dc0, 0x4c80, 0x8c41,
        0x4400, 0x84c1, 0x8581, 0x4540, 0x8701, 0x47c0, 0x4680, 0x8641,
        0x8201, 0x42c0, 0x4380, 0x8341, 0x4100, 0x81c1, 0x8081, 0x4040
    ];

    pub fn new() -> Self {
        return Self {
           checksum: 0,
           device_cmd: 0,
           device_status: 0,
           length: 0,
           data: [0u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize],
           internal_counter: 0,
           length_counter: 0,
        }
    }

    pub fn getDataArray(&self) -> [u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize] {
        return self.data;
    }

    pub fn addData(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() > FlemPacket::FLEM_MAX_DATA_SIZE as usize {
            self.device_status = FlemStatus::FLEM_ERROR_PACKET_OVERFLOW;
            Err("Error: Data exceeded packet size, no data added, device_status updated to
                FlemStatus::FLEM_ERROR_PACKET_OVERFLOW")
        }else{
            for i in 0..data.len() {
                self.data[i] = data[i];
            }
            self.length = data.len() as u16;

            Ok(())
        }
    }

    fn validate(&mut self) -> bool {
        let crc = self.checksum(false);
        return crc == self.checksum;
    }

    pub fn add_byte(&mut self, interface: &FlemInterface, byte: &u8) -> Result<bool, &'static str> {
        let retval = FlemAddByte {
            info: FlemStatus::FLEM_INFO_CONSTRUCTING_PACKET,
        };

        //We can't use 10..max_data_u32 below, and have to use
        //10..=max_data_u32. Therefore, subtract 1 from max data to make things
        //work.
        const max_data_u32: u32 = FlemPacket::FLEM_HEADER_SIZE as u32 + FlemPacket::FLEM_MAX_DATA_SIZE as u32;
        
        match self.internal_counter {
            0 => { self.checksum = *byte as u16; },
            1 => { self.checksum |= (*byte as u16) << 8; },
            2 => { self.device_cmd = *byte as u32; },
            3 => { self.device_cmd |= (*byte as u32) << 8; },
            4 => { self.device_cmd |= (*byte as u32) << 16; },
            5 => { self.device_cmd |= (*byte as u32) << 24; },
            6 => { self.device_status = *byte as u16; },
            7 => { self.device_status = (*byte as u16) << 8; },
            8 => { self.length = *byte as u16; },
            9 => { 
                self.length |= (*byte as u16) << 8; 
                self.length_counter = 0;
                if self.length == 0 {
                    //Packet has no data
                    if self.validate() {
                        //Packet has no data and is valid
                        (interface.valid_handler)(interface, self);
                        return Ok(true);
                    } else{
                        //Packet has no data and is not valid
                        (interface.error_handler)(interface, self, FlemStatus::FLEM_ERROR_CHECKSUM);
                        return Ok(false);
                    }
                }
            },
            10..=max_data_u32 => {
                println!("{}", self.internal_counter);
                self.data[self.length_counter] = *byte;
                self.length_counter += 1;
                if self.length as usize == self.length_counter {
                       //Length # of bytes received, validate 
                        if self.validate() {
                            //Packet is valid
                            (interface.valid_handler)(interface, self);
                            return Ok(true);
                        } else{
                            //Packet is not valid
                            (interface.error_handler)(interface, self, FlemStatus::FLEM_ERROR_CHECKSUM);
                            return Ok(false);
                        }
                    }
            }, 
            _ => { return Err("Packet overflow"); }
        }

        self.internal_counter += 1;

        //TODO: add byte to vector
        //TODO: serialize byte to FlemPacket

        Ok(false)
    }

    pub fn get_next_byte(&mut self, interface: &FlemInterface) -> Result<u8, &'static str> {
       let bytes = self.asU8Array();
       let cnt = self.internal_counter;
       const max_size: u32 = (FlemPacket::FLEM_MAX_DATA_SIZE + 
           FlemPacket::FLEM_HEADER_SIZE as u16) as u32;
       match cnt {
           0..=max_size => {
               let byte = bytes[self.internal_counter as usize];
               self.internal_counter += 1;
               Ok(byte)
           },
           _ => {
               self.internal_counter = 0;
               Err("Warning: End of packet reached, resetting internal packet counter")
           }
       }
    }

    pub fn setCommand(&mut self, command: u32) {
        self.device_cmd = command;
    }

    pub fn setStatus(&mut self, status: u16) {
        self.device_status = status;
    }

    pub fn asU8Array(&self) -> &[u8] {
        let stream: &[u8] = unsafe {  
            ::core::slice::from_raw_parts(
                (self as *const FlemPacket) as *const u8, 
                self.length() as usize
            )
        };

        return stream;
    }

    pub fn checksum(&mut self, store: bool) -> u16 {
        let mut crc: u16 = 0;
        let bytes: &[u8] = self.asU8Array();
        let psize: u16 = bytes.len() as u16;
        
        //Skip the first 2 checksum bytes
        for i in 2..psize {
            let ptr = bytes[i as usize] as u16;    
            let lut_index = (crc ^ ptr) as u8;
            let mut tmp_crc = FlemPacket::crc16_tab[lut_index as usize];
            tmp_crc ^= crc >> 8;
            crc = tmp_crc;
        }

        if store {
            self.checksum = crc;
        }

        return crc;
    }

    pub fn reset(&mut self) {
        self.checksum = 0;
        self.device_cmd = 0;
        self.device_status = 0;
        self.length = 0;
        self.internal_counter = 0;
        self.data = [0u8; FlemPacket::FLEM_MAX_DATA_SIZE as usize];
        self.length_counter = 0;
    }

    pub fn length(&self) -> u16 {
        let mut x: u16 = FlemPacket::FLEM_HEADER_SIZE as u16;
        x += self.length as u16;
        return x;
    }
}
