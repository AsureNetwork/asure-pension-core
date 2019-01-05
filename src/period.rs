use chrono::{DateTime, Utc};
use std::collections::linked_list::LinkedList;
use crate::transaction::Transaction;

pub struct Period{
    pub transactions:LinkedList<Transaction>,
    pub date: DateTime<Utc>

}





#[cfg(test)]
mod tests {

}