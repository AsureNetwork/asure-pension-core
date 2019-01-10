use crate::token::*;

pub struct Wallet {
    pub dpt: Token,
    pub eth: f64,
    pub pension_eth: f64,
    pub tokens: Vec<Token>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            dpt: Token { name: String::from("DTP"), amount: 0.0 },
            eth: 10000.0,
            pension_eth: 0.0,
            tokens: Vec::new(),
        }
    }

    pub fn get_total_pension_eth(&self) -> f64 {
        self.pension_eth
    }

    pub fn get_total(&self) -> f64 {
        self.eth + self.pension_eth
    }

    //todo:
    pub fn get_dpt(&self) -> u128 {
        //self.tokens.iter.fold(0.0, |acc, token| acc + token.amount)
        return 1 as u128;
    }
}




