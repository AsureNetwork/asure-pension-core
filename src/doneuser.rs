use crate::pensioner::Pensioner;

pub struct DoneUser {
    pub pensioner: Pensioner,
}

impl DoneUser {
    pub fn new(pensioner: Pensioner) -> Self {
        DoneUser {
            pensioner
        }
    }
}