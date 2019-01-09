use std::vec::Vec;

use crate::transaction::*;

pub struct Period {
    //pub index: u64,
    pub txs: Vec<Transaction>,

}

impl Period {
    pub fn new() -> Period {
        Period {
            //index: 0,
            txs: Vec::new(),
        }
    }

    pub fn create_tx(&mut self) {
        self.txs.push(Transaction::new());
    }
}

#[cfg(test)]
mod tests {}