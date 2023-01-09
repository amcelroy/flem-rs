#![no_std]

pub struct FlemRequest;
pub struct FlemConfig;
pub struct FlemResponse;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlemStatus {
    Ok,
    PacketReceived,
    PacketBuilding,
    GetByteFinished,
    VersionLength,
    PacketOverflow,
    HeaderBytesNotFound,
    PacketConstruction,
    GetByteIssue,
    ChecksumError,
}

const FLEM_ID_VERSION_SIZE: usize = 30;
pub struct FlemDataId {
    version: [char; FLEM_ID_VERSION_SIZE as usize],
    max_packet_size: usize,
}

impl FlemDataId {
    pub fn new(version: &str, packet_size: usize) -> FlemDataId {
        let mut id = FlemDataId {
            version: ['\0'; FLEM_ID_VERSION_SIZE as usize],
            max_packet_size: packet_size,
        };

        let version_size: usize = version.len();

        assert!(version_size < FLEM_ID_VERSION_SIZE, "Version should be 30 characters or less");

        for a in 0..version_size {
            id.version[a as usize] = version.as_bytes()[a as usize] as char;
        }
        id
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct FlemPacket<const T: usize> {
    header: u16,
    checksum: u16,
    request: u8,
    response: u8,
    length: u16,
    data: [u8; T],
    internal_counter: u32,
    data_length_counter: usize,
    status: FlemStatus,
}

impl FlemResponse {
    pub const SUCCESS: u8 = 0;
    pub const FLEM_BUSY: u8 = 1;
    pub const UNKNOWN_REQUEST: u8 = 0xFD;
    pub const CHECKSUM_ERROR: u8 = 0xFE;
    pub const ERROR: u8 = 0xFF;
}

impl FlemRequest {    
    pub const EVENT: u8 = 0;
    pub const ID: u8 = 1;
    pub const IDLE: u8 = 0xFF;
}

const FLEM_HEADER_SIZE: usize = 8;
const FLEM_HEADER: u16 = 0x5555;
const CRC16_TAB: [u16; 256] = [
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

impl<const T: usize> FlemPacket<T> {
    pub fn new() -> Self {
        assert!(T < u16::MAX as usize, "<T> should be u16::MAX or less"); // Bounds check T, must be less than u16::MAX
        return Self {
           header: 0,
           checksum: 0,
           request: 0,
           response: 0,
           length: 0,
           data: [0u8; T],
           internal_counter: 0,
           data_length_counter: 0,
           status: FlemStatus::Ok,
        }
    }

    pub fn respond_data(&mut self, request: u8, data: &[u8]) -> Result<(), FlemStatus> {
        self.request = request;
        match self.add_data(data) {
            Ok(_) => {
                self.response = FlemResponse::SUCCESS;
                self.pack();
                Ok(())
            },
            Err(e) => {
                self.response = FlemResponse::ERROR;
                Err(e)
            }
        }
    }

    pub fn respond_error(&mut self, request: u8, response: u8 ) {
        self.request = request;
        self.response = response;
        self.pack();
    }

    pub fn response_id(&mut self, id: &FlemDataId) {
        self.request = FlemRequest::ID;
        self.response = FlemResponse::SUCCESS;
        
        // TODO: Copy ID over

        self.pack();
    }

    pub fn pack(&mut self) {
        self.checksum(true);
        self.header = FLEM_HEADER;
    }

    pub fn get_data_array(&self) -> [u8; T] {
        return self.data;
    }

    pub fn add_data(&mut self, data: &[u8]) -> Result<(), FlemStatus> {
        if data.len() > T {
            self.status = FlemStatus::PacketOverflow;
            Err(FlemStatus::PacketOverflow)
        }else{
            for i in 0..data.len() {
                self.data[i] = data[i];
            }
            self.length = data.len() as u16;

            self.status = FlemStatus::Ok;
            Ok(())
        }
    }

    pub fn validate(&mut self) -> bool {
        let crc = self.checksum(false);
        return crc == self.checksum;
    }

    pub fn add_byte(&mut self, byte: &u8) -> FlemStatus {      
        let local_internal_counter = self.internal_counter;

        match local_internal_counter {
            0 => { 
                if *byte != 0x55 {
                    self.internal_counter = 0;
                    self.status = FlemStatus::HeaderBytesNotFound;
                    return self.status;
                }
                self.header = *byte as u16; 
            },
            1 => { 
                if *byte != 0x55 {
                    self.internal_counter = 0;
                    self.status = FlemStatus::HeaderBytesNotFound;
                    return self.status;
                }
                self.header |= (*byte as u16) << 8; 
            },
            2 => { self.checksum = *byte as u16; },
            3 => { self.checksum |= (*byte as u16) << 8; },
            4 => { self.request = *byte; },
            5 => { self.response = *byte; },
            6 => { self.length = *byte as u16; },
            7 => { 
                self.length |= (*byte as u16) << 8; 
                self.data_length_counter = 0;
                self.status = FlemStatus::PacketReceived;
                // if self.length == 0 {
                //     //Packet has no data
                //     if self.validate() {
                //         //Packet has no data and is valid
                //         (interface.valid_handler)(self);
                //         self.reset_counters();
                //         return Ok(());
                //     } else{
                //         //Packet has no data and is not valid
                //         (interface.error_handler)(self, FlemResponse::CHECKSUM_ERROR);
                //         self.reset_counters();
                //         return Err(FlemStatus::ChecksumError);
                //     }
                // }
            },
            i if (FLEM_HEADER_SIZE as u32 <= i && i <= T as u32) => {
                self.data[self.data_length_counter] = *byte;
                self.data_length_counter += 1;
                if self.length as usize == self.data_length_counter {
                    self.status = FlemStatus::PacketReceived;
                    // //Length # of bytes received, validate 
                    // if self.validate() {
                    //     //Packet is valid
                    //     (interface.valid_handler)(self);
                    //     self.reset_counters();
                    //     return Ok(());
                    // } else{
                    //     //Packet is not valid
                    //     (interface.error_handler)(self, FlemResponse::CHECKSUM_ERROR);
                    //     self.reset_counters();
                    //     return Err(FlemStatus::ChecksumError);
                    // }
                }
            }, 
            _ => {  self.status = FlemStatus::PacketOverflow; }
        }

        self.internal_counter += 1;
        self.status = FlemStatus::PacketBuilding;

        self.status
    }

    /// This function treats the entire packet as a byte array and uses internal
    /// counters to determine the next byte. Keep calling this until either an
    /// error occurs or 
    pub fn get_next_byte(&mut self) -> Result<u8, FlemStatus> {
       let bytes = self.as_u8_array();
       let cnt = self.internal_counter;
       match cnt {
           i if (i <= T as u32) => {
               let byte = bytes[self.internal_counter as usize];
               self.internal_counter += 1;
               self.status = FlemStatus::Ok;
               Ok(byte)
           },
           _ => {
               self.internal_counter = 0;
               self.status = FlemStatus::GetByteFinished;
               Err(self.status)
           }
       }
    }

    /// Sets the Flem request field
    pub fn set_request(&mut self, request: u8) {
        self.request = request;
    }

    /// Gets the Flem request field
    pub fn get_request(&self) -> u8 {
        self.request
    }

    /// Sets the Flem response field
    pub fn set_response(&mut self, response: u8) {
        self.response = response;
    }

    /// Gets the Flem response field
    pub fn get_response(&self) -> u8 {
        self.response
    }

    pub fn get_status(&mut self) -> FlemStatus {
        self.status
    }

    /// Returns the data as a u8 array
    pub fn as_u8_array(&self) -> &[u8] {
        let stream: &[u8] = unsafe {  
            ::core::slice::from_raw_parts(
                (self as *const FlemPacket<T>) as *const u8, 
                self.length() as usize
            )
        };

        return stream;
    }

    /// Computes a CRC16 IBM style checksum on the packet, except the header
    /// and checksum bytes
    pub fn checksum(&mut self, store: bool) -> u16 {
        let mut crc: u16 = 0;
        let bytes: &[u8] = self.as_u8_array();
        let psize: u16 = bytes.len() as u16;
        
        //Skip the first 4 bytes, 2 header and 2 checksum
        for i in 4..psize {
            let ptr = bytes[i as usize] as u16;    
            let lut_index = (crc ^ ptr) as u8;
            let mut tmp_crc = CRC16_TAB[lut_index as usize];
            tmp_crc ^= crc >> 8;
            crc = tmp_crc;
        }

        if store {
            self.checksum = crc;
        }

        return crc;
    }

    /// Resets the internal byte counters
    fn reset_counters(&mut self) {
        self.internal_counter = 0;
        self.data_length_counter = 0;
    }

    /// Resets the packet to all 0's, but does not clear the data array
    pub fn reset_lazy(&mut self) {
        self.reset(true);
    }

    /// Resets the packet to all 0's, including the data array
    pub fn reset(&mut self, lazy: bool) {
        self.checksum = 0;
        self.request = 0;
        self.response = 0;
        self.length = 0;
        self.internal_counter = 0;
        self.status = FlemStatus::Ok;
        if !lazy {
            self.data = [0u8; T];
        }
        self.data_length_counter = 0;
    }

    pub fn length(&self) -> u16 {
        let mut x: u16 = FLEM_HEADER_SIZE as u16;
        x += self.length as u16;
        return x;
    }
}
