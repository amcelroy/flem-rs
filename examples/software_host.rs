use flem::{
    Packet,
    traits::Host
};
use std::{
    sync::mpsc,
};

const PACKET_SIZE: usize = 512;

struct FlemSoftwareHost{
    listening: bool, 
}

impl FlemSoftwareHost {
    pub fn new() -> Self {
        FlemSoftwareHost {
            listening: false,
        }
    }
}

impl Host<PACKET_SIZE> for FlemSoftwareHost {
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

    fn listen(&mut self, rx_sleep_time_ms: u64, tx_sleep_time_ms: u64,) -> (Receiver<Packet<T>>, Sender<Packet<T>>) {
        let (tx, tx) = mpsc::channel::<Packet<T>>::new();
        
        self.listening = true;

        // Tx Thread - Transmit packets to the "device"
        thread::spawn(move |tx| {
            while self.listening {
                //if 

                thread::sleep(Duration::from_millis(rx_sleep_time_ms));
            }
        });

        (rx, tx)
    }

    fn unlisten(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn main() {
    let mut host = FlemSoftwareHost::new();

    let (rx, tx) = host.listen(10, 10);

    loop {
        match rx.recv() {
            Ok(packet) => {
                println!("Packet received!");
            }
            Err(_) => {
                println!("Packet error occurred!");
            }
        }
    }




}