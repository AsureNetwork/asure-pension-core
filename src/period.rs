use std::collections::linked_list::LinkedList;

use crate::transaction::*;

pub struct Period {
    pub index: u64,
    pub txs: LinkedList<Transaction>,

}

impl Period {
    pub fn new() -> Period {
        Period {
            index: 0,
            txs: LinkedList::new(),
        }
    }

    pub fn create_tx(mut self) {
        self.txs.push_front(Transaction::new());
    }
}

#[cfg(test)]
mod tests {}