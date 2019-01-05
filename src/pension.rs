use crate::transaction::*;
use crate::user::*;

use std::collections::linked_list::LinkedList;

pub struct Pension{
    pub period: u64,
    pub txs: Transaction,
    pub total_eth: u64,
    pub total_month_eth: u64,
    pub total_dpt: u64,
    pub total_month_dpt: u64,
    pub total_retirement_dpt: u64,
    pub users:LinkedList<User>
}

impl Pension{

    pub fn new()->Pension{
        Pension{
            period: 0,
            txs: Transaction::new(),
            total_eth: 0,
            total_month_eth: 0,
            total_dpt: 0,
            total_month_dpt: 0,
            total_retirement_dpt: 0,
            users: LinkedList::new()
        }
    }

    pub fn create_users(&self){

    }

    pub fn pay(&self){

    }

    pub fn payout(&self){

    }

    pub fn activate_retirement(&self)-> bool{
        return false;
    }

    pub fn calculate_points(&self)-> u128{
        return 0;
    }

    pub fn end(&self){

    }


}
