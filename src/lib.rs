
pub mod core{
    pub fn run(){
        println!("Run Hello, world!");
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
