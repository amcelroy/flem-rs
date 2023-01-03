use flem;
use flem::*;

use core::cell::Cell;
use std::sync::atomic::{AtomicU8, Ordering};

// Size of packet, including the Header (8 byte)
// So a size of 108 would leave 100 bytes for data
const FLEM_PACKET_SIZE: usize = 100;

// For embedded systems, the packet will be assembled in an interrupt, which 
// means the valid_handler function will be called from the interrupt space. 
// We whould cache the value of the request to handle in the process_handler
// function which will occur in the main() thread space.
static mut RxRequest: AtomicU8 = AtomicU8::new(0);

// Implement our own custom Request commands
struct FlemRequestProjectX;

impl FlemRequestProjectX {
    const GET_DATA: u8 = 10;
}

fn flem_error_handler(_rx: &mut FlemPacket<FLEM_PACKET_SIZE>, status: u8) {
    println!("Error detected: {}", status);
}

fn flem_valid_handler(rx: &mut FlemPacket<FLEM_PACKET_SIZE>) {
    println!("Client Packet Received");
    unsafe {
        let req = rx.get_request();
        RxRequest.swap(req, Ordering::Relaxed);
    }
}

fn flem_process_handler(request: u8, 
    response_packet: &mut FlemPacket<FLEM_PACKET_SIZE>,
    request_packet: &mut FlemPacket<FLEM_PACKET_SIZE>,
) {
    // Get request packet payload, if needed
    let data = request_packet.get_data_array();

    // Decode data if needed

    match request {
        0 => {
            // Do nothing here
        },
        FlemRequest::ID => {
            let data: [u8; 11] = [0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0];
            response_packet.respond_data(request, &data).unwrap(); 
        },
        FlemRequestProjectX::GET_DATA => {            
            let data: [u8; 11] = [0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0];
            response_packet.respond_data(request, &data).unwrap();
        },
        FlemRequest::IDLE => {
            // TODO: 
            // Do nothing?
            // WFI?
        },
        _ => { 
            response_packet.respond_error(request, FlemResponse::UNKNOWN_REQUEST); 
        }
    };
}

fn main() {
    let flem_id = FlemDataId::new( "0.0.1", FLEM_PACKET_SIZE);

    let flem_client_interface = flem::FlemInterface::<FLEM_PACKET_SIZE> {
        id: flem_id,
        error_handler: flem_error_handler,
        valid_handler: flem_valid_handler,
        process_handler: flem_process_handler,
    };

    let mut host_tx = flem::FlemPacket::<FLEM_PACKET_SIZE>::new();

    let mut client_tx = flem::FlemPacket::<FLEM_PACKET_SIZE>::new();
    let mut client_rx = flem::FlemPacket::<FLEM_PACKET_SIZE>::new();

    println!("Packet data length: {}", client_rx.get_data_array().len());
    
    println!("Packet length: {}", client_rx.length());

    host_tx.reset(false);
    host_tx.set_request(FlemRequestProjectX::GET_DATA);
    host_tx.pack(); // Pack runs checksum and after that it is ready to send

    // Simulates byte-by-byte tranmission
    for _i in 0..host_tx.length() {
        let next_byte = host_tx.get_next_byte().unwrap();

        //Transmit from host / receive on client

        client_rx.add_byte(&flem_client_interface, &next_byte).unwrap();
    }

    // The Client's main thread should be running and handle process requests.
    // In RTIC, the rx and tx packet should be global resources
    let mut request: u8 = 0;
    unsafe {
        request = RxRequest.load(Ordering::Relaxed);
    }

    (flem_client_interface.process_handler)(request, &mut client_tx, &mut client_rx);

    // Finished with the Rx Packet
    client_rx.reset_lazy();

    // TODO: Send out client_tx packet here
    // TODO: Reset

    unsafe {
        RxRequest.swap(flem::FlemRequest::IDLE, Ordering::Relaxed);
    }


}
