pub struct BigNum {
    data: Vec<u64>
}

impl BigNum {
    fn new() -> BigNum {
        BigNum {
            data: Vec::new()
        }
    }
}
