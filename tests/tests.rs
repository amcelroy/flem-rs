use flem;

#[cfg(test)]
mod tests {
    
    const FLEM_PACKET_SIZE: usize = 108;

    #[test]
    fn reset() {
        let rx = flem::FlemPacket::<FLEM_PACKET_SIZE>::new();
        let tx = flem::FlemPacket::<FLEM_PACKET_SIZE>::new();

        //println!("Length: {}", rx.getData().len());
    }

    #[test]
    fn checksum() {
        let rx = flem::FlemPacket::<FLEM_PACKET_SIZE>::new();

    }

    #[test]
    fn size_check() {
        // TODO: Check sizes here
    }
}
