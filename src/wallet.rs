use crate::token::Token;

#[derive(Debug)]
pub struct Wallet {
    pub dpt: Token,
    pub eth: f64,
    pub pension_eth: f64,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            dpt: Token { name: String::from("DTP"), amount: 0.0 },
            eth: 10000.0,
            pension_eth: 0.0,
        }
    }

    pub fn get_total_eth(&self) -> f64 {
        self.eth + self.pension_eth
    }
    pub fn get_total_dpt(&self) -> f64 {
        self.dpt.amount
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::*;

    #[test]
    fn should_get_total_eth() {
        let mut wallet = Wallet::new();
        wallet.pension_eth = 10000.0;
        assert_eq!(wallet.get_total_eth(), 20000.0);
    }


    #[test]
    fn should_get_dept() {
        let mut wallet = Wallet::new();
        wallet.dpt.amount = 1000.0;

        assert_eq!(wallet.get_total_dpt(), 1000.0);
    }
}





