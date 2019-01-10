pub struct Token {
    pub name: String,
    pub amount: f64,
}

impl Token{
    pub fn new() -> Token {
        Token{
            name: String::from("ETH"),
            amount: 0.0
        }
    }
}
