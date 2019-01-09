pub struct Transaction {
    pub period: u32,
    pub amount: f64,
}

impl Transaction {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new(period: u32, amount: f64) -> Transaction {
        Transaction {
            period,
            amount
        }
    }
}


#[cfg(test)]
mod tests {}