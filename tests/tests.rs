use flem;

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

        assert_eq!(tx.length(), flem::FLEM_HEADER_SIZE as u16 + payload.len() as u16, "Packet length incorrect");
        
        let x = tx.as_u8_array();

        for byte in tx.as_u8_array() {
            match rx.add_byte(byte) {
                flem::Status::PacketReceived => {
                    
                },
                flem::Status::PacketBuilding => {
                    byte_counter += 1;
                }
                _ => {
                    assert!(true, "Should not be hit");
                }
            }
        }
        assert_eq!(byte_counter, flem::FLEM_HEADER_SIZE as usize + payload.len(), "Not all bytes were sent");
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
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as u16, "Size should be 14 (i.e. just the header)");

        let smaller_payload = [10 as u8; 60];
        assert!(rx.add_data(&smaller_payload).is_ok(), "Payload is smaller than allocated packet, this SHOULD NOT cause an error");
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as u16 + smaller_payload.len() as u16, "Size should be 68 (i.e. header + smaller_payload.len)");

        rx.reset_lazy();
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as u16, "Size should be 14 (i.e. just the header)");
        let just_right_payload = [10 as u8; FLEM_PACKET_SIZE];
        assert!(rx.add_data(&just_right_payload).is_ok(), "Payload is exactly the size of the allocated packet, this SHOULD NOT cause an error");
        assert_eq!(rx.length(), flem::FLEM_HEADER_SIZE as u16 + just_right_payload.len() as u16, "Size should be 116 (i.e. header + just_right_payload.len)");
    }
}
