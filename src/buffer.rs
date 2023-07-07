#[derive(Debug, Clone, Copy)]
pub enum DataBufferErrors {
    NotEnoughRoomInBuffer,
    ConversionWouldOverflow,
}

/// Convert an f32 to a little endian byte array and add it to a buffer
pub fn f32_to_le_buffer(
    data: f32,
    buffer: &mut [u8],
    offset: &mut usize,
) -> Result<(), DataBufferErrors> {
    let bytes = data.to_le_bytes();

    if *offset + bytes.len() > buffer.len() {
        Err(DataBufferErrors::NotEnoughRoomInBuffer)
    } else {
        buffer[*offset] = bytes[0];
        buffer[*offset + 1] = bytes[1];
        buffer[*offset + 2] = bytes[2];
        buffer[*offset + 3] = bytes[3];
        *offset += 4;
        Ok(())
    }
}

/// Convert a little endian byte array from a buffer to an f32
pub fn le_buffer_to_f32(buffer: &[u8], offset: &mut usize) -> Result<f32, DataBufferErrors> {
    let mut tmp = [0 as u8; 4];

    if *offset + tmp.len() > buffer.len() {
        Err(DataBufferErrors::ConversionWouldOverflow)
    } else {
        tmp[0] = buffer[*offset];
        tmp[1] = buffer[*offset + 1];
        tmp[2] = buffer[*offset + 2];
        tmp[3] = buffer[*offset + 3];
        *offset += 4;
        Ok(f32::from_le_bytes(tmp))
    }
}

/// Convert an u32 to a little endian byte array and add it to a buffer
pub fn u32_to_le_buffer(
    data: u32,
    buffer: &mut [u8],
    offset: &mut usize,
) -> Result<(), DataBufferErrors> {
    let bytes = data.to_le_bytes();

    if *offset + bytes.len() > buffer.len() {
        Err(DataBufferErrors::NotEnoughRoomInBuffer)
    } else {
        buffer[*offset] = bytes[0];
        buffer[*offset + 1] = bytes[1];
        buffer[*offset + 2] = bytes[2];
        buffer[*offset + 3] = bytes[3];
        *offset += 4;
        Ok(())
    }
}

/// Convert a little endian byte array from a buffer to an u32
pub fn le_buffer_to_u32(buffer: &[u8], offset: &mut usize) -> Result<u32, DataBufferErrors> {
    let mut tmp = [0 as u8; 4];

    if *offset + tmp.len() > buffer.len() {
        Err(DataBufferErrors::ConversionWouldOverflow)
    } else {
        tmp[0] = buffer[*offset];
        tmp[1] = buffer[*offset + 1];
        tmp[2] = buffer[*offset + 2];
        tmp[3] = buffer[*offset + 3];
        *offset += 4;
        Ok(u32::from_le_bytes(tmp))
    }
}

/// Convert an i32 to a little endian byte array and add it to a buffer
pub fn i32_to_le_buffer(
    data: i32,
    buffer: &mut [u8],
    offset: &mut usize,
) -> Result<(), DataBufferErrors> {
    let bytes = data.to_le_bytes();

    if *offset + bytes.len() > buffer.len() {
        Err(DataBufferErrors::NotEnoughRoomInBuffer)
    } else {
        buffer[*offset] = bytes[0];
        buffer[*offset + 1] = bytes[1];
        buffer[*offset + 2] = bytes[2];
        buffer[*offset + 3] = bytes[3];
        *offset += 4;
        Ok(())
    }
}

/// Convert a little endian byte array from a buffer to an i32
pub fn le_buffer_to_i32(buffer: &[u8], offset: &mut usize) -> Result<i32, DataBufferErrors> {
    let mut tmp = [0 as u8; 4];

    if *offset + tmp.len() > buffer.len() {
        Err(DataBufferErrors::ConversionWouldOverflow)
    } else {
        tmp[0] = buffer[*offset];
        tmp[1] = buffer[*offset + 1];
        tmp[2] = buffer[*offset + 2];
        tmp[3] = buffer[*offset + 3];
        *offset += 4;
        Ok(i32::from_le_bytes(tmp))
    }
}

/// Convert an u32 to a little endian byte array and add it to a buffer
pub fn u16_to_le_buffer(
    data: u16,
    buffer: &mut [u8],
    offset: &mut usize,
) -> Result<(), DataBufferErrors> {
    let bytes = data.to_le_bytes();

    if *offset + bytes.len() > buffer.len() {
        Err(DataBufferErrors::NotEnoughRoomInBuffer)
    } else {
        buffer[*offset] = bytes[0];
        buffer[*offset + 1] = bytes[1];
        *offset += 2;
        Ok(())
    }
}

/// Convert a little endian byte array from a buffer to an u16
pub fn le_buffer_to_u16(buffer: &[u8], offset: &mut usize) -> Result<u16, DataBufferErrors> {
    let mut tmp = [0 as u8; 2];

    if *offset + tmp.len() > buffer.len() {
        Err(DataBufferErrors::ConversionWouldOverflow)
    } else {
        tmp[0] = buffer[*offset];
        tmp[1] = buffer[*offset + 1];
        *offset += 2;
        Ok(u16::from_le_bytes(tmp))
    }
}

/// Convert an i16 to a little endian byte array and add it to a buffer
pub fn i16_to_le_buffer(
    data: i16,
    buffer: &mut [u8],
    offset: &mut usize,
) -> Result<(), DataBufferErrors> {
    let bytes = data.to_le_bytes();

    if *offset + bytes.len() > buffer.len() {
        Err(DataBufferErrors::NotEnoughRoomInBuffer)
    } else {
        buffer[*offset] = bytes[0];
        buffer[*offset + 1] = bytes[1];
        *offset += 2;
        Ok(())
    }
}

pub fn le_buffer_to_i16(buffer: &[u8], offset: &mut usize) -> Result<i16, DataBufferErrors> {
    let mut tmp = [0 as u8; 2];

    if *offset + tmp.len() > buffer.len() {
        Err(DataBufferErrors::ConversionWouldOverflow)
    } else {
        tmp[0] = buffer[*offset];
        tmp[1] = buffer[*offset + 1];
        *offset += 2;
        Ok(i16::from_le_bytes(tmp))
    }
}
