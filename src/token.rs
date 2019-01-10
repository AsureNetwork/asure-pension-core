pub struct Token {
    pub name: String,
    pub amount: u128,
}

impl Token{
    pub fn new() -> Token{
        Token{
            name: String::from("ETH"),
            amount: 0
        }
    }
}
