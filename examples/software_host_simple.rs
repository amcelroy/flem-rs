use flem::{
    Packet,
    DataId,
    traits::Channel
};
use std::{
    time::Duration,
    sync::{
        Arc,
        Mutex, 
        mpsc::{self, Sender, Receiver}
    },
    thread,
};

const PACKET_SIZE: usize = 512;

#[derive(Clone)]
struct FlemSoftwareHost<const PACKET_SIZE: usize>{
    listening: Arc<Mutex<bool>>, 
    flem_packet_handler: Option<fn(&Packet<PACKET_SIZE>) -> Packet<PACKET_SIZE>>
}

impl<const PACKET_SIZE: usize> FlemSoftwareHost<PACKET_SIZE> {
    pub fn new() -> Self {
        FlemSoftwareHost {
            listening: Arc::new(Mutex::new(false)),
            flem_packet_handler: None,
        }
    }
}

impl<const PACKET_SIZE: usize> Channel<PACKET_SIZE> for FlemSoftwareHost<PACKET_SIZE> {
    type Error = ();

    fn list_devices(&self) -> Vec<String> {
        let mut devices = Vec::<String>::new();
        devices.push(String::from("Software Host"));
        devices
    }

    fn connect(&mut self, device: &String) -> Result<(), Self::Error> {
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn listen(&mut self, rx_sleep_time_ms: u64, tx_sleep_time_ms: u64,) -> (Sender<Packet<PACKET_SIZE>>, Receiver<Packet<PACKET_SIZE>>) {        
        // Tx packets are marshalled into a single queue, and dispatched over hardware.
        let (tx_packet_from_program, packet_to_transmit) = mpsc::channel::<flem::Packet<PACKET_SIZE>>();

        // Rx data is coming off of hardware, usually a byte at a time, and needs to be constructed into a packet and validated before passing back into the program
        let (validated_packet, rx_packet_to_program) = mpsc::channel::<flem::Packet<PACKET_SIZE>>();
        
        *self.listening.lock().unwrap() = true;

        let listening_clone = self.listening.clone();

        let device_flem_handler = self.flem_packet_handler.clone();

        // Tx Thread - Transmit packets to the "device"
        let device_handle = thread::spawn(move || {            
            while *listening_clone.lock().unwrap() {
                // Check if there is a packet to transmit, use recv_timeout to prevent a blocking thread
                if let Ok(tx_packet) = packet_to_transmit.recv_timeout(Duration::from_millis(tx_sleep_time_ms)) {
                    if device_flem_handler.is_none() {
                        println!("Packet handler not set, working as a loop-back");
                        validated_packet.send(tx_packet);
                    }else{
                        println!("Packet handler set, calling handler");
                        let handler = device_flem_handler.as_ref().unwrap();
                        let response = handler(&tx_packet);
                        validated_packet.send(response);
                    }
                }
            }
        });

        (tx_packet_from_program, rx_packet_to_program)
    }

    fn unlisten(&mut self) -> Result<(), Self::Error> {
        *self.listening.lock().unwrap() = false;
        Ok(())
    }
}

fn main() {
    let mut host = FlemSoftwareHost::<PACKET_SIZE>::new();

    // Our "device" FLEM handler
    fn device_flem_handler(packet: &Packet<PACKET_SIZE>) -> Packet<PACKET_SIZE> {
        let mut response = Packet::<PACKET_SIZE>::new();

        match packet.get_request() {
            flem::request::ID => {
                let id = DataId::new("Emulated Target", 0, 0, 1, PACKET_SIZE);

                // Respond with ID
                response.set_request(flem::request::ID);
                response.set_response(flem::response::SUCCESS);
                response.pack_id(&id, true);
            },
            _ => {
                response.set_request(packet.get_request());
                response.set_response(flem::response::UNKNOWN_REQUEST);
                response.pack();
            }
        }

        response
    }

    // Configure the packet handler
    host.flem_packet_handler = Some(device_flem_handler);

    let (tx, rx) = host.listen(10, 10);

    let mut packet = flem::Packet::<PACKET_SIZE>::new();

    packet.set_request(flem::request::ID);
    packet.pack();

    tx.send(packet).unwrap();

    loop {
        if let Ok(packet) = rx.recv_timeout(Duration::from_millis(25)) 
        {
            println!("Received packet: {:?}", packet);

             // Do stuff with the packet
            match packet.get_request() {
                flem::request::ID => {
                    let id = DataId::from(&packet.get_data()).unwrap();
                    println!(
                        "DataId Message: {}, max packet size: {}, Major: {}, Minor: {}, Patch: {}", 
                        String::from_iter(id.get_name().iter()),
                        id.get_max_packet_size(),
                        id.get_major(),
                        id.get_minor(),
                        id.get_patch()
                    );
                },
                _ => {
                    // Unknown request
                }
            }

            host.unlisten().unwrap();
            break;
        }
    }
}