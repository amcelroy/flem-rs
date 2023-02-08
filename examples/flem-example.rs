use flem;
use flem::*;

use std::sync::atomic::{AtomicU8, Ordering};

// Size of packet, including the Header (8 byte)
// So a size of 108 would leave 100 bytes for data
const FLEM_PACKET_SIZE: usize = 100;

// Implement our own custom Request commands
struct FlemRequestProjectX;

impl FlemRequestProjectX {
    const GET_DATA: u8 = 10;
}

fn main() {
    let flem_id = DataId::new( "0.0.1", FLEM_PACKET_SIZE);

    let mut host_tx = flem::Packet::<FLEM_PACKET_SIZE>::new();

    let mut client_tx = flem::Packet::<FLEM_PACKET_SIZE>::new();
    let mut client_rx = flem::Packet::<FLEM_PACKET_SIZE>::new();

    println!("Packet data length: {}", client_rx.get_data_array().len());
    
    println!("Packet length: {}", client_rx.length());

    host_tx.reset(false);
    host_tx.set_request(FlemRequestProjectX::GET_DATA);
    host_tx.pack(); // Pack runs checksum and after that it is ready to send

    // Simulates byte-by-byte tranmission
    for _i in 0..host_tx.length() {
        
        let mut next_byte: u8 = 0;
        match host_tx.get_next_byte() {
            Ok(byte) => {
                next_byte = byte;
            },
            Err(status) => {
                assert!(false, "Error calling get_next_byte()");
            }
        }

        //Transmit from host / receive on client
        match client_rx.add_byte(&next_byte) {
            Status::PacketReceived => {
                println!("Packet received successfully!");
            },
            Status::PacketBuilding => {
                // No issues, keep going
            }
            _ => {
                println!("Packet error occurred!");
            }
        }
    }

    match client_rx.get_request() {
        Request::EVENT => {

        },
        Request::ID => {

        },
        Request::IDLE => {

        },
        FlemRequestProjectX::GET_DATA => {
            println!("Request received: FlemRequestProjectX::GET_DATA");
        },
        _ => {

        }
    }
}
