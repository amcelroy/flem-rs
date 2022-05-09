use flem;
use flem::FlemPacket;
use flem::FlemInterface;
use flem::FlemDataId;

fn flem_error_handler(i: &FlemInterface, p: &mut FlemPacket, status: u16) {
    println!("Error detected: {}", status);
}

fn flem_valid_handler(i: &FlemInterface, p: &mut FlemPacket) {
    println!("Packet success, work your magic!");
}

fn main() {
    let special_functions = [0u32; 10];

    let flem_interface = flem::FlemInterface {
        id: FlemDataId::new("FLEM Rust Demo", "0.0.1", &[0u32; 10]).unwrap(),
        error_handler: flem_error_handler,
        valid_handler: flem_valid_handler,
    };

    let x = flem::FlemCommands::FLEM_SET_OFFSET;    

    let mut rx = flem::FlemPacket::new();
    let mut tx = flem::FlemPacket::new();

    println!("Packet data length: {}", rx.getDataArray().len());
    
    println!("Packet length: {}", rx.length());

    tx.setStatus(0);
    tx.setCommand(1);

    let crc = tx.checksum(true);
    println!("Packet CRC: {}", crc);

    println!("Packet as u8: {:?}", tx.asU8Array());
    
    for i in 0..tx.length() {
        let next_byte = tx.get_next_byte(&flem_interface).unwrap();
        println!("TX Packet byte: {}", next_byte);
        rx.add_byte(&flem_interface, &next_byte);
    }

    println!("Example with max data payload");
             
    //Try again with some data!
    tx.reset();
    rx.reset();

    tx.setCommand(1);
    let mut some_data: Vec<u8> = Vec::with_capacity(FlemPacket::FLEM_MAX_DATA_SIZE as usize);
   
    for a in 0..FlemPacket::FLEM_MAX_DATA_SIZE {
        some_data.push(a as u8);
    }

    tx.addData(&some_data);
    tx.checksum(true);

    println!("Packet length: {}", tx.length());

    for i in 0..tx.length() {
        let next_byte = tx.get_next_byte(&flem_interface).unwrap();
        rx.add_byte(&flem_interface, &next_byte);
    }

}
