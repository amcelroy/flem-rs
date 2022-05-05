use flem;

fn main() {
    let x = flem::FlemCommands::FLEM_SET_OFFSET;    

    let mut rx = flem::FlemPacket::new();
    let tx = flem::FlemPacket::new();

    println!("Packet data length: {}", rx.getDataArray().len());
    
    println!("Packet length: {}", rx.length());

    rx.setStatus(1);
    rx.setCommand(2);


    println!("Packet as u8: {:?}", rx.asU8Array());
}
