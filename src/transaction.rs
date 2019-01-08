pub struct Transaction {
    pub amount: u64,
}

impl Transaction {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Transaction {
        Transaction {
            amount: 0
        }
    }
}


#[cfg(test)]
mod tests {}