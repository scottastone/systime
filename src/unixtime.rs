pub fn unix_timestamp_micros() -> u128 {
    let now = std::time::SystemTime::now();
    return now.duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros();
}