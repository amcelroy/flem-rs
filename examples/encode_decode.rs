use flem;
use flem::buffer::le_buffer_to_u32;
use flem::traits::{DataInterface, DataInterfaceErrors};
use flem::*;

// Size of packet, including the Header (8 byte)
// So a size of 108 would leave 100 bytes for data
const FLEM_PACKET_SIZE: usize = 100;

pub mod request_projectx {
    pub const GET_DIAGNOSTICS: u16 = 10;
}

/// Task times in milliseconds
pub struct Diagnostics {
    task_1: u32,
    task_2: u32,
    task_3: u32,
}

impl Diagnostics {
    const DIAGNOSTICS_SIZE_BYTES: usize = 12;

    pub fn new() -> Self {
        Diagnostics {
            task_1: 100,
            task_2: 10,
            task_3: 25,
        }
    }
}

impl<const T: usize> DataInterface<T> for Diagnostics {
    // Encodes a struct into a packet
    fn encode(&self, packet: &mut Packet<T>) -> Result<(), traits::DataInterfaceErrors> {
        // Check if the packet can even hold the struct, add more checks here
        if T < Diagnostics::DIAGNOSTICS_SIZE_BYTES {
            Err(DataInterfaceErrors::IncorrectDataLength)
        } else {
            packet.add_data(&self.task_1.to_le_bytes()).unwrap();
            packet.add_data(&self.task_2.to_le_bytes()).unwrap();
            packet.add_data(&self.task_3.to_le_bytes()).unwrap();
            Ok(())
        }
    }

    // Decode a packet to a struct
    fn decode(&mut self, packet: &Packet<T>) -> Result<&Self, traits::DataInterfaceErrors> {
        // Check if the packet can even hold the struct, add more checks here
        if T < Diagnostics::DIAGNOSTICS_SIZE_BYTES {
            Err(DataInterfaceErrors::IncorrectDataLength)
        } else {
            let mut offset = 0;
            let data = packet.get_data();

            self.task_1 = le_buffer_to_u32(&data, &mut offset).unwrap();
            self.task_2 = le_buffer_to_u32(&data, &mut offset).unwrap();
            self.task_3 = le_buffer_to_u32(&data, &mut offset).unwrap();
            Ok(self)
        }
    }
}

fn main() {
    let mut host_tx = flem::Packet::<FLEM_PACKET_SIZE>::new();
    let mut host_rx = flem::Packet::<FLEM_PACKET_SIZE>::new();

    let mut client_rx = flem::Packet::<FLEM_PACKET_SIZE>::new();
    let mut client_tx = flem::Packet::<FLEM_PACKET_SIZE>::new();

    // Default values of - task_1: 100, task_2: 10, task_3: 25
    let client_diagnostics = Diagnostics::new();

    host_tx.reset_lazy();
    host_tx.set_request(request_projectx::GET_DIAGNOSTICS);
    host_tx.pack();

    // Transmit request
    for byte in host_tx.bytes() {
        match client_rx.construct(*byte) {
            Ok(_) => {
                println!("Packet received on client");
            }
            // More error checking here, if needed
            Err(status) => {
                // We expect Status::PacketBuilding
                if status != Status::PacketBuilding {
                    assert!(true, "An error shouldn't have occurred in this example");
                }
            }
        }
    }

    client_tx.reset_lazy();
    client_tx.set_request(client_rx.get_request());
    client_tx.set_response(flem::response::SUCCESS);
    // Encodes the struct into the packet
    client_diagnostics.encode(&mut client_tx).unwrap();
    // Compute and store the checksum
    client_tx.pack();

    for byte in client_tx.bytes() {
        match host_rx.construct(*byte) {
            Ok(_) => {
                println!("Packet received on host");
            }
            Err(result) => {
                // We expect Status::PacketBuilding
                if result != Status::PacketBuilding {
                    assert!(true, "An error shouldn't have occurred in this example");
                }
            }
        }
    }

    // Create a new struct to hold the decoded data
    let mut host_diagnostics = Diagnostics {
        task_1: 0,
        task_2: 0,
        task_3: 0,
    };

    // Decode the packet into the struct
    host_diagnostics.decode(&host_rx).unwrap();

    assert_eq!(
        client_diagnostics.task_1, host_diagnostics.task_1,
        "Task 1 not the same"
    );
    assert_eq!(
        client_diagnostics.task_2, host_diagnostics.task_2,
        "Task 2 not the same"
    );
    assert_eq!(
        client_diagnostics.task_3, host_diagnostics.task_3,
        "Task 3 not the same"
    );

    println!(
        "Client: Task 1: {}, Task 2: {}, Task 3: {}",
        client_diagnostics.task_1, client_diagnostics.task_2, client_diagnostics.task_3
    );

    println!(
        "Host: Task 1: {}, Task 2: {}, Task 3: {}",
        host_diagnostics.task_1, host_diagnostics.task_2, host_diagnostics.task_3
    );

    println!("Transmission successful!");
}
