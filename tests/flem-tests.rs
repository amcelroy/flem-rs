use  flem;

#[cfg(test)]
mod tests {
    
    #[test]
    fn reset() {
        let rx = flem::FlemPacket::new();
        let tx = flem::FlemPacket::new();

        println!("Length: {}", rx.getData().len());
    }
}
