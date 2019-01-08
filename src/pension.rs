use crate::period::*;
use crate::user::*;

pub struct Pension {
    pub period: Vec<Period>,
    pub total_eth: u64,
    pub total_month_eth: u64,
    pub total_dpt: u64,
    pub total_month_dpt: u64,
    pub total_retirement_dpt: u64,
    pub users: Vec<User>,
}

impl Pension {
    pub fn new() -> Pension {
        Pension {
            period: Vec::new(),
            total_eth: 0,
            total_month_eth: 0,
            total_dpt: 0,
            total_month_dpt: 0,
            total_retirement_dpt: 0,
            users: Vec::new(),
        }
    }

    pub fn create_users(&mut self, count: u32) {
        for _ in 0..count {
            self.users.push(User::new());
        }
    }

    pub fn pay(&self) {}

    pub fn payout(&self) {}


    pub fn calculate_points(&self) -> u128 {
        return 0;
    }

    pub fn end(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::pension::*;

    #[test]
    fn create_users() {
        let mut pension = Pension::new();
        pension.create_users(5);

        assert_eq!(pension.users.len(), 5);
    }
}
