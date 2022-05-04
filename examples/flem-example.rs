use flem;

fn main() {
    let x = flem::FlemCommands::FLEM_SET_OFFSET;    

    let rx = flem::FlemPacket::new();
    let tx = flem::FlemPacket::new();

    println!("Packet data length: {}", rx.getDataArray().len());
    
    println!("Packet length: {}", rx.length());

}
