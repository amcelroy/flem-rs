use flem;
use flem::*;

static mut TX_PACKET: Option<FlemPacket> = None;

fn flem_error_handler(_i: &FlemInterface, _p: &mut FlemPacket, status: u16) {
    println!("Error detected: {}", status);
}

fn flem_valid_handler(_i: &FlemInterface, p: &mut FlemPacket) {
    println!("Packet success, work your magic!");

    let result = match p.get_command() {
        FlemCommands::FLEM_COMMAND_GET_SMART_DRAIN => {
            //Note: In C, I have heap allocated rx and tx packets that are 
            //global to the flem.c project and only accessed within the C file
            //and not from other sources.
            
            unsafe {
                let mut pp = TX_PACKET.unwrap();
                pp.reset();
                pp.set_command(p.get_command());
                pp.set_status(FlemStatus::FLEM_INFO_PROCESSED);
                pp.checksum(true);
                TX_PACKET = Some(pp);
            }

            //let mut tx_packet = FlemPacket::new();
            //tx_packet.reset();
            //tx_packet.set_command(p.get_command());
            //tx_packet.set_status(FlemStatus::FLEM_INFO_PROCESSED);
            //tx_packet.checksum(true);
            Ok(())
        },
        _ => { Err("Command not recognized") }
    };

    result.unwrap();
}

fn main() {
    unsafe {
        TX_PACKET = Some(FlemPacket::new());
    }

    let flem_interface = flem::FlemInterface {
        id: FlemDataId::new("FLEM Rust Demo", "0.0.1", &[0u32; 10]).unwrap(),
        error_handler: flem_error_handler,
        valid_handler: flem_valid_handler,
    };

    let mut rx = flem::FlemPacket::new();
    let mut tx = flem::FlemPacket::new();

    println!("Packet data length: {}", rx.get_data_array().len());
    
    println!("Packet length: {}", rx.length());

    tx.set_status(0);
    tx.set_command(FlemCommands::FLEM_COMMAND_GET_SMART_DRAIN);

    let crc = tx.checksum(true);
    println!("Packet CRC: {}", crc);
    
    for _i in 0..tx.length() {
        let next_byte = tx.get_next_byte(&flem_interface).unwrap();
        rx.add_byte(&flem_interface, &next_byte).unwrap();
    }

    println!("Example with max data payload");
             
    //Try again with some data!
    tx.reset();
    rx.reset();

    tx.set_command(FlemCommands::FLEM_COMMAND_GET_SMART_DRAIN);
    let mut some_data: Vec<u8> = Vec::with_capacity(FlemPacket::FLEM_MAX_DATA_SIZE as usize);
   
    for a in 0..FlemPacket::FLEM_MAX_DATA_SIZE {
        some_data.push(a as u8);
    }

    tx.add_data(&some_data).unwrap();
    tx.checksum(true);

    println!("Packet length: {}", tx.length());

    for _i in 0..tx.length() {
        let next_byte = tx.get_next_byte(&flem_interface).unwrap();
        rx.add_byte(&flem_interface, &next_byte).unwrap();
    }

}
