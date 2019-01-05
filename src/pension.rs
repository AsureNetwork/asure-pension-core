use std::collections::linked_list::LinkedList;
use crate::user::User;
use crate::period::Period;

pub struct Pension{
    pub total_eth:u64,
    pub total_month_eth:u64,
    pub total_dpt: u64,
    pub total_month_dpt: u64,
    pub total_retirement_dpt: u64,
    pub users: LinkedList<User>,
    pub current_period: Period
}

