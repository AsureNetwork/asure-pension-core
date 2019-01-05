
pub struct User{
    pub name:String,
    pub wallet:Wallet,
    pub pension_status: u128,
    pub pension_payment_months:u128,
    pub pension_recived_months:u128,
    pub eth:u128,
    pub pension_eth: u128,
    pub total:u128,
    pub dpt: u128,
    pub activated_dtp: u128,
}


pub struct Wallet{
    pub eth:i64,
    pub pension_eth:i64,
    pub tokens: LinkedList<Token>
}

pub struct Token{
    pub amount:i64,
    pub created: DateTime
}


#[cfg(test)]
mod tests {

}