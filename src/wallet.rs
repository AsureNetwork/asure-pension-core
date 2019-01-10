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
            dpt: Token { name: String::from("DTP"), amount: 0 },
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

    pub fn get_dpt(&mut self) -> u128 {
        self.tokens.iter_mut().fold(0, |acc, token| acc + token.amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::*;
    use crate::token::*;

    #[test]
    fn should_get_total_eth() {
        let mut wallet = Wallet::new();
        wallet.pension_eth = 10000.0;
        assert_eq!(wallet.get_total(), 20000.0);
    }

    #[test]
    fn should_get_total_pension_eth() {
        let mut wallet = Wallet::new();
        wallet.pension_eth = 10000.0;
        assert_eq!(wallet.get_total_pension_eth(), 10000.0);
    }

    #[test]
    fn should_get_dept() {
        let mut wallet = Wallet::new();
        let mut token_1 = Token::new();
        token_1.amount = 1000;

        let mut token_2 = Token::new();
        token_2.amount = 2000;

        let mut token_3 = Token::new();
        token_3.amount = 3000;

        wallet.tokens.push(token_1);
        wallet.tokens.push(token_2);
        wallet.tokens.push(token_3);

        assert_eq!(wallet.get_dpt(), 6000);
    }
}





