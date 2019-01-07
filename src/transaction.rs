pub struct Transaction {
    data: i64
}

impl Transaction {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Transaction {
        Transaction {
            data: 10
        }
    }
}


#[cfg(test)]
mod tests {}