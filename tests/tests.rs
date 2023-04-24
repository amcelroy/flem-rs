#[cfg(test)]
mod tests {
    
    const FLEM_PACKET_SIZE: usize = 108;

    #[test]
    fn sending() {
        const REQUEST: u8 = 0xF;

        let mut rx = flem::Packet::<FLEM_PACKET_SIZE>::new();
        let mut tx = flem::Packet::<FLEM_PACKET_SIZE>::new();

        let mut payload = [0 as u8; FLEM_PACKET_SIZE];
        for i in 0..FLEM_PACKET_SIZE {
            payload[i] = i as u8;
        }

        tx.set_request(REQUEST);
        assert_eq!(REQUEST, tx.get_request(), "Requests do not match");
        assert!(tx.add_data(&payload).is_ok(), "Payload is exactly packet size, SHOULD NOT cause error");
        assert_eq!(tx.checksum(true), 50848, "Checksum mismatch");
        tx.pack();

        let mut byte_counter = 0;

        assert_eq!(tx.length(), flem::FLEM_HEADER_SIZE as usize + payload.len(), "Packet length incorrect");
        
        let _x = tx.bytes();

        let mut packet_received = false;
        for byte in tx.bytes() {
            match rx.add_byte(*byte) {
                flem::Status::PacketReceived => {
                    byte_counter += 1;
                    packet_received = true;
                },
                flem::Status::PacketBuilding => {
                    byte_counter += 1;
                }
                _ => {
                    assert!(true, "Should not be hit");
                }
            }
        }
        assert!(packet_received, "Packet not detected as received");
        assert_eq!(byte_counter, flem::FLEM_HEADER_SIZE as usize + payload.len(), "Not all bytes were sent");

        let tx_u8_array = tx.bytes();
        let rx_u8_array = rx.bytes();

        for i in 0..tx_u8_array.len() {
            assert_eq!(tx_u8_array[i], rx_u8_array[i], "Tx packet not the same as Rx packet at byte {}", i);
        }
    }

    #[test]
    fn checksum() {
        let mut rx = flem::Packet::<FLEM_PACKET_SIZE>::new();

        rx.set_request(flem::Request::ID);
        let checksum = rx.checksum(true);

        assert_eq!(checksum, 64513, "Checksum mismatch");
        assert_eq!(checksum, rx.get_checksum(), "Checksum mismatch");
    }


    #[test]
    fn size_check() {
        let mut rx = flem::Packet::<FLEM_PACKET_SIZE>::new();

        assert_eq!(rx.length(), 8, "Size should be 14 (i.e. just the header)");

        let payload = [10 as u8; FLEM_PACKET_SIZE + 1];
        assert!(rx.add_data(&payload).is_err(), "Payload is larger than allocated packet, this SHOULD cause an error");
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as usize, "Size should be 14 (i.e. just the header)");

        let smaller_payload = [10 as u8; 60];
        assert!(rx.add_data(&smaller_payload).is_ok(), "Payload is smaller than allocated packet, this SHOULD NOT cause an error");
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as usize + smaller_payload.len(), "Size should be 68 (i.e. header + smaller_payload.len)");

        rx.reset_lazy();
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as usize, "Size should be 14 (i.e. just the header)");
        let just_right_payload = [10 as u8; FLEM_PACKET_SIZE];
        assert!(rx.add_data(&just_right_payload).is_ok(), "Payload is exactly the size of the allocated packet, this SHOULD NOT cause an error");
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as usize + just_right_payload.len(), "Size should be 116 (i.e. header + just_right_payload.len)");
    }

    #[test]
    fn documentation_test_get_byte() {
        use flem::{Packet};
        use heapless;
        const PACKET_SIZE: usize = 64; // 64 byte packet 
        const FLEM_EXAMPLE_REQUEST: u8 = 0xF;
        
        let mut rx = Packet::<PACKET_SIZE>::new();
        let mut tx = Packet::<PACKET_SIZE>::new();
        
        let data = [0 as u8; PACKET_SIZE];
        
        /* Add data as needed to the data buffer */
        tx.add_data(&data).unwrap();
        tx.set_request(FLEM_EXAMPLE_REQUEST);
        tx.pack();

        /* Send data */
        let mut tx_fifo_queue = heapless::spsc::Queue::<u8, 8>::new();
        let mut keep_sending = true;
        let mut packet_received = false;
        let mut status = flem::Status::Ok;
        
        while keep_sending {
            if !tx_fifo_queue.is_full() && status != flem::Status::GetByteFinished {
                // Keep sending data
                match tx.get_byte() {
                    Ok(byte) => {
                        tx_fifo_queue.enqueue(byte).unwrap();
                    },                
                    Err(x) => {
                        /* Tx code should stop transmitting */
                        status = x;
                    }
                }
            }else{
                // Queue is full, Tx the data, Rx on the other end
                while !tx_fifo_queue.is_empty() {
                    match rx.add_byte(tx_fifo_queue.dequeue().unwrap()) {
                        flem::Status::PacketReceived => {
                            packet_received = true;
                            keep_sending = false;
                        },
                        _ => {
                            /* Catch other statuses here on the Rx side */
                        }
                    }
                }
            }
        }

        assert!(packet_received, "Packet should have been transferred");

        // This test is redundant, since the checkums passed, still nice to see

        let rx_bytes = rx.bytes();
        let tx_bytes = tx.bytes();

        for i in 0..rx_bytes.len() {
            assert_eq!(rx_bytes[i], tx_bytes[i], "Rx and Tx packets don't match");
        }

    }              
}
