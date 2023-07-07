![Flem Build and Tests](https://github.com/amcelroy/flem-rust/actions/workflows/rust.yml/badge.svg)

# FLEM Rust

FLEM stands for Flexible, Light-weight, Embedded Messaging and is a Little 
Endian messaging protocol intended for use in communicating with embedded 
systems targets over numerous types of busses. The host makes requests to the 
client (typically the embedded target). The client processes the requests and 
responds. The client can asynchronously send an Event packet that the host can 
deal with as needed. Together, a host and a client make a partner.

## Concepts

At its core, FLEM has packets composed of a header, and a data payload. The 
header is 8 bytes and consists of:
- Header - 2 bytes - Should always be a value of 0x5555
- Checksum - 2 bytes - CRC-16 (IBM) of the packet (exludes the header and 
checksum bytes)
- Request - 1 byte - A value from 0 to 255 that indicates what the client 
should do. There are some reserved values, see below.
- Response - 1 byte - A value of 0 to 255 that indicates additional information
 about the client.
- Length - 2 bytes - Number of bytes being transmitted in the `data` buffer. 
Can be 0 to u16::MAX.
- Data - An array **up to** u16::MAX_SIZE. This is set when creating a new 
packet and represents the maximum length of data that can be sent or received; 
anything less than the max can also be transmitted. The Packet is allocated with
MAX_SIZE bytes as a buffer, something to consider on low memory systems.

## Reseting a packet
A packet should be reset before reuse. There are 2 methods provided to do this:
- `reset()` - Performs a full reset of the packet, by zeroing out all bytes
including zeroing out the data bytes.
- `lazy_reset()` - Resets the Checksum, Request, Response, and Length bytes.

## Packing a packet
After data is added, the packet should be packed where the header, checksum,
request, response, length are all configured and the checksum computed on the
following bytes:
- Request
- Response
- Length
- Data

In Rust, a few convenience functions are provided to `lazy_reset` the packet, 
add data, set the header bytes, and perform the pack operation:
- `pack_data` - Adds data to a packet with a Response byte of `SUCCESS` and the
Request byte set by the user.
- `pack_error` - Adds data to a packet with Request and Response bytes specified 
by the user. If no data needs to be transmitted, use an empty data array `&[]`.
- `pack_event` - Adds data to a packet with the Request byte set as an `EVENT`
and the Response byte set to a user defined value.

__Note__: If no data is to be transmitted, set data to an empty data array 
`&[]`.

## Header
The header is a value of 0x5555 and represents a set of bytes that can be 
scanned quickly to determine the start of a packet. This may be expanded in the 
future to allow for other Header byte patterns.

## Checksum
A CRC-16 (IBM) checksum that can be used to ensure the data was transmitted and
received without error. The checksum calculation **does not** include the 
header or the checksum bytes; ensure they are either zero or skipped if
implemented in another language.

## Request
Typically, a host sends a 1-byte request to a client. A request doesn't need to 
have any data payload to it, in which case a simple request packet is 8 bytes. 
A response packet should always echo the request to ensure the partner device 
can double check and route the response correctly. 

__Note__, the client is not required to respond. The client responses help 
ensure commands were sent, processed, successful (or not) and may help validate
that the hardware tranmission of data is performing as intended.

Requests are mostly left up to the user to implement, though there are some 
pre-defined ones.

- Event (0x00) - A request of 0 indicates the partner is sending out data without 
a request being sent first. If this is the case, the Response Byte will contain 
the u8 command of the event, which should be pre-determined between host and 
client.
- Id (0x01) - Each device using FLEM should implement a `DataId` struct that 
indicates a version, serial number, name, or some other information (30 bytes, 
char) and a u16 indicating the partners max packet size.  This requires that 
the client / host use packet sizes of at least 32 bytes. Smaller Ids can be 
used, or not responded to, but it is up to the user to implement.

Our company has a seperate project that has all of the responses, requests, and
events for each project in a different Rust sub-module. This way, our 
communication protocols are all in one spot and revision controlled for use in
future projects.

## Response
Responses are 1-byte codes that indicate the status of the client.

#### Response byte if the Request is an Event: 
There are some reserved non-event responses:
- Success - 0x00 - Nothing went wrong
- Busy - 0x01 - Indicates the the partner is busy
- PacketOverflow - 0xFC - Too much data was sent to the partner and it exceeded
the packet buffer.
- UnknownRequest - 0xFD - The request wasn't recognized by the partner
- ChecksumError - 0xFE - Checksum did not compute correctly
- Error - 0xFF - Unspecified error when processing a request

#### Response byte if the Request is an Event:
If the partner is sending an event, the Response byte represents a command that 
the partner would like to execute or convery to the host. As with Requests, the
Response byte code in the case of an event should be pre-determined between the
host and the client.

## Length
Two bytes inidcating the amount of data to expect in the packets data field. 
This can be 0 to u16::MAX, though typically it would be something smaller. 

## Data
The packet data payload. Can be 0 to u16::MAX bytes. 

## Examples

See `examples/flem-example.rs` for a host to client request and a client to host
 response.
