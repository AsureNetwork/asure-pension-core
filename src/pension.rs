use std::collections::linked_list::LinkedList;

use crate::user::*;
use crate::period::*;

pub struct Pension {
    pub period: LinkedList<Period>,
    pub total_eth: u64,
    pub total_month_eth: u64,
    pub total_dpt: u64,
    pub total_month_dpt: u64,
    pub total_retirement_dpt: u64,
    pub users: LinkedList<User>,
}

impl Pension {
    pub fn new() -> Pension {
        Pension {
            period: LinkedList::new(),
            total_eth: 0,
            total_month_eth: 0,
            total_dpt: 0,
            total_month_dpt: 0,
            total_retirement_dpt: 0,
            users: LinkedList::new(),
        }
    }

    pub fn create_users(mut self, mut count: u32) {
        while count > 0 {
            self.users.push_front(User::new());
            count -= 1;
        }
    }

    pub fn pay(&self) {}

    pub fn payout(&self) {}

    pub fn activate_retirement(&self) -> bool {
        return false;
    }

    pub fn calculate_points(&self) -> u128 {
        return 0;
    }

    pub fn end(&self) {}
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use crate::pension::*;

        #[test]
        fn create_users() {
            let pension = Pension::new();
            pension.create_users(5);
            //todo
            //assert_eq!(pension.users.len(), 5);
        }
    }
}
