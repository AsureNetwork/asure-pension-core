pub struct Settings {
    pub current_contribution_value_degree: f64,
    pub current_contribution_value: f64,
    pub years: u64,
}


impl Settings {
    // A public constructor method
    #[warn(dead_code)]
    pub fn new() -> Settings {
        Settings {
            current_contribution_value_degree: 10.0,
            current_contribution_value: 1.0,
            years: 40,
        }
    }

    //fn is_odd(n: u32) -> bool {
    //  n % 2 == 1
    //}
}