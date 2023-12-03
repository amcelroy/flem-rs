![Flem Build and Tests](https://github.com/amcelroy/flem-rust/actions/workflows/rust.yml/badge.svg)

# FLEM Rust 0.6.2

FLEM stands for Flexible, Light-weight, Embedded Messaging and is a Little 
Endian messaging protocol intended for use in communicating with embedded 
systems targets over numerous types of buses. 

## Changelog 

### Changelog 0.6.2
- Added feature = ["std"]
- Added `Channel` trait. This trait requires features = ["std"]. It serves as a set of traits that can be used
to implement different hardware or to emulate a device. It uses the `std` library for threading and `mpsc` channels.
- Added examples to show how to emulate a device use the `Channel` trait.

### Changelog 0.6.1
- Added fmt::Debug trait to Packet that prints the header, checksum, request, response, length, and status.

### Changelog 0.6.0 (from 0.5.0)
- Requests are now 2 byte u16 instead of 1 byte u8
- Responses are now 2 byte u16 instead of 1 byte u8
- Events are no longer a concept. Instead, each device should check incoming packets 
for requests and handle those as normal. This should simplify code and prevent
accidentally mixing requests / responses / events.
- Built in responses have changed: Busy was removed, and SUCCESS was moved from 0x00 to 0x0001. ASYNC is now 0x0000 and represents a packet being sent without asking and is the default option upon a packet reset
or instantiation.
- Updated unit tests and examples

## Concepts

At its core, FLEM has packets composed of a header, and a data payload. The 
header is 10 bytes and consists of:
- Header - 2 bytes - Should always be a value of 0x5555
- Checksum - 2 bytes - CRC-16 (IBM) of the packet (excludes the header and 
checksum bytes)
- Request - 2 byte - A value from 0 to 65535 that indicates what the client 
should do. There are some reserved values, see below.
- Response - 2 byte - A value of 0 to 65535 that indicates additional information
 about the client.
- Length - 2 bytes - Number of bytes being transmitted in the `data` buffer. 
Can be 0 to u16::MAX.
- Data - An array **up to** u16::MAX_SIZE. This is set when creating a new 
packet and represents the maximum length of data that can be sent or received; 
anything less than the max can also be transmitted. The Packet is allocated with
MAX_SIZE bytes as a buffer, something to consider on low memory systems.

## Resetting a packet
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
Typically, a host sends a 2-byte request to a client. A request doesn't need to 
have any data payload, in which case a simple request packet is 10 bytes. 
A response packet should always echo the request to ensure the partner device 
can double check and route the response correctly. 

__Note__, the client is not required to respond. The client responses help 
ensure commands were sent and processed successful (or not). It will also help validate
that the hardware transmission of data is performing as intended via the checksum.

Requests are mostly left up to the user to implement, though there is one 
pre-defined:

- Id (0x01) - Each device using FLEM should implement a `DataId` struct that 
indicates a version, serial number, name, or some other information (30 bytes, 
char) and a u16 indicating the partners max packet size.  This requires that 
the client / host use packet sizes of at least 30 bytes. Smaller Ids can be 
used, or not responded to, but it is up to the user to implement.

Our company has a separate project that has all of the responses and requests
for each project in a different Rust sub-module. Typically, each project has
something like:
```
pub mod host_requests {
    pub const READ_CONFIG: u16 = ...;
    pub const WRITE_CONFIG: u16 = ...;
}

pub mod client_requests {
    pub const INTERRUPT: u16 = ...;
    pub const DATA_ACQUIRED: u16 = ...;
}
```

## Response
Responses are 2-byte codes that indicate the status of the partner device, if needed.  

#### Response byte if the Request is an Event: 
There are some reserved non-event responses:
- ASYNC - 0x0000 - The packet is being sent without asking
- SUCCESS - 0x0001 - Nothing went wrong processing the request, the request is likewise echoed in the response packet.
- UNKNOWN_REQUEST - 0xFFFD - The request wasn't recognized by the partner
- CHECKSUM_ERROR - 0xFFFF - Checksum did not compute correctly

## Length
Two bytes indicating the amount of data to expect in the packets data field. 
This can be 0 to u16::MAX, though typically it would be something smaller. 

## Data
The packet data payload. Can be 0 to u16::MAX bytes. 

## Traits

FLEM offers a `DataInterface` trait that can be implemented on a struct that 
allow the user to encode and decode the struct into a FLEM packet. In general,
check that the struct can fit in the packet when encoding, and that the packet 
has enough data to fit into the struct when decoding. 

Once this check is done, the data can be moved into or out of the struct, making
sure that the encoding and decoding order is consistent. See `examples/traits.rs`
for an example. There are convenience functions in `src/buffer.rs` that can be 
used when decoding, with unit tests for these functions in `tests/tests.rs`.


## Examples

See `examples/example.rs` for a host to client request and a client to host
response.
