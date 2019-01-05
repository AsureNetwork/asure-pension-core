
pub struct User{
    pub name:String,
    pub wallet:Wallet,
    pub PensionStatus: u128,
    pub PensionPaymentMonths:u128,
    pub PensionRecivedMonths:u128,
    pub ETH:u128,
    pub pensionETH: u128,
    pub total:u128,
    pub dpt: u128,
    pub activatedDPT: u128,
}


pub struct Wallet{
    pub eth:i64,
    pub pensionETH:i64,
    pub tokens: LinkedList<Token>
}

pub struct Token{
    pub Amount:i64,
    //pub Created:Date
}


#[cfg(test)]
mod tests {

}