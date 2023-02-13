use flem;
use flem::*;

use std::{iter::FromIterator};

// Size of packet, including the Header (8 byte)
// So a size of 108 would leave 100 bytes for data
const FLEM_PACKET_SIZE: usize = 100;

// Implement our own custom Request commands
struct FlemRequestProjectX;

impl FlemRequestProjectX {
    const GET_DATA: u8 = 10;
}

fn main() {
    let client_flem_id = DataId::new( "0.0.1", FLEM_PACKET_SIZE);

    // There should typically be at least 1 packet each for Rx / Tx
    let mut host_tx = flem::Packet::<FLEM_PACKET_SIZE>::new();
    let mut host_rx = flem::Packet::<FLEM_PACKET_SIZE>::new();

    // There should typically be at least 1 packet each for Rx / Tx
    let mut client_tx = flem::Packet::<FLEM_PACKET_SIZE>::new();
    let mut client_rx = flem::Packet::<FLEM_PACKET_SIZE>::new();

    println!("Packet data length: {}", client_rx.get_data().len());
    
    println!("Packet length: {}", client_rx.length());

    host_tx.reset(false);
    host_tx.set_request(flem::Request::ID); // Change this for different responses from the client
    host_tx.pack(); // Pack runs checksum and after that it is ready to send

    // Simulates byte-by-byte tranmission
    for _i in 0..host_tx.length() {        
        let mut next_byte: u8 = 0;
        match host_tx.get_byte() {
            Ok(byte) => {
                next_byte = byte;
            },
            Err(status) => {
                assert!(false, "get_byte() finished");
            }
        }

        /* Hardware bus / protocol (I2C, UART, etc) goes here */

        //Transmit from host / receive on client
        match client_rx.add_byte(next_byte) {
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
    host_tx.reset_lazy(); // Reset the host_tx so it can be used again

    /* Process request on the client side */
    client_tx.reset_lazy();
    match client_rx.get_request() {
        Request::EVENT => {
            // Clients typically send events, but maybe not in your case!
        },
        Request::ID => {
            client_tx.response_id(&client_flem_id, true); 
            client_tx.pack();
        },
        FlemRequestProjectX::GET_DATA => {
            // Custom command implemented for this project (Project X)
            let project_x_data = [0 as u8; 40];
            client_tx.respond_data(client_rx.get_request(), &project_x_data);
            println!("Request received: FlemRequestProjectX::GET_DATA");
        },
        _ => {
            client_tx.respond_error(client_rx.get_request(), flem::Response::UnknownRequest as u8);
        }
    }
    client_rx.reset_lazy(); // Reset the client_rx packet so it can be used again

    /* Send response back to host */
    let mut host_size_data_id = DataId::new("", 0);
    for byte in client_tx.bytes() {
        match host_rx.add_byte(*byte) {
            flem::Status::PacketReceived => {
                // Determine what to do with the received packet
                match host_rx.get_request() {
                    Request::EVENT => {
                        // Hosts typically consume events, but maybe not in your case!
                    },
                    Request::ID => {
                        host_size_data_id = flem::DataId::from(&host_rx.get_data()).unwrap();
                        println!("DataId Message: {}, max packet size: {}", String::from_iter(host_size_data_id.get_version().iter()), host_size_data_id.get_max_packet_size());
                    },
                    FlemRequestProjectX::GET_DATA => {
                        // Custom command implemented for this project (Project X)
                        // Do something with the requested data
                    },
                    _ => {
                        // Uh oh
                    }
                }

                host_rx.reset_lazy(); // Reset the host_rx so it can be used again
            },
            _ => {
                /* Catch other errors here */
            }
        }
    }
    client_tx.reset_lazy(); // Reset the client_tx so it can be used again

}
