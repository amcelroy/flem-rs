# FLEM Rust

FLEM stands for Flexible, Light-weight, Embedded Messaging and is a Little Endian messaging protocol intended for use in communicating with embedded systems targets over numerous types of busses. The host makes requests to the client (typically the embedded target). The client processes the requests and responds. The client can asynchronously send an Event packet that the host can deal with as needed. Together, a host and a client make a partner.

## Concepts

At its core, FLEM has packets composed of a header, and a data payload. The header is 8 bytes and consists of:
- Header - 2 bytes - Should always be a value of 0x5555
- Checksum - 2 bytes - CRC-16 (IBM) of the packet (exludes the header and checksum bytes)
- Request - 1 byte - A value from 0 to 255 that indicates what the client should do. There are some reserved values, see below.
- Response - 1 byte - A value of 0 to 255 that indicates additional information about the client.
- Length - 2 bytes - Number of bytes being transmitted in the `data` buffer. Can be 0 to u16::MAX.
- Data - An array **up to** u16::MAX_SIZE. This is set when creating a new packet and represents the maximum length of data that can be sent or received;  anything less than the max can also be transmitted.

### Header
The header is a value of 0x5555 and represents a set of bytes that can be scanned quickly to determine the start of a packet.

### Checksum
An CRC-16 (IBM) checksum that can be used to ensure the data was transmitted and received without error. The checksum calculation **does not** include the header or the checksum bytes.

### Request
Typically, a host send a 1-byte request to a client. A request doesn't need to have any data payload to it, in which case a simple request packet is 8 bytes. A response packet should always echo the request to ensure the partner device can double check and route the response correctly.

Requests are mostly left up to the user to implement, though there are some pre-defined ones.

- Event (0x00) - A request of 0 indicates the partner is sending out data without a request being sent first. It is up to the user to decode and route event data.
- Id (0x01) - Each device using FLEM should implement a `DataId` struct that indicates a version, serial, name, or some other information (30 bytes, char) and a u16 indicating the partners max packet size.  This requires that the client / host use packet sizes of at least 32 bytes. Smaller Ids can be used, or not responded to, but it is up to the user to implement.

### Response
Responses are 1-byte codes that indicate the status of the partner. There are some reserved responses:
- Success - 0x00 - Nothing went wrong
- Busy - 0x01 - Indicates the the partner is busy
- PacketOverflow - 0xFC - Too much data was sent to the partner and it exceeded the packet buffer.
- UnknownRequest - 0xFD - The request wasn't recognized by the partner
- ChecksumError - 0xFE - Checksum did not compute correctly
- Error - 0xFF - Unspecified error when processing a request

### Length
Two bytes inidcating the amount of data to expect in the packets data field. This can be 0 to u16::MAX, though typically it would be something smaller. 

### Data
The packet data payload. Can be 0 to u16::MAX bytes. 

## Documentation

See `docs` for documentation.

## Examples

See `examples/flem-example.rs` for a host to client request and a client to host response.