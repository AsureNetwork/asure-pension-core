use crate::new::contributor::Contributor;
use crate::new::pensioner::Pensioner;
use crate::new::doneuser::DoneUser;

pub enum User {
    Contributor(Contributor),
    Pensioner(Pensioner),
    Done(DoneUser),
}

impl User {
    pub fn new() -> Self {
        User::Contributor(Contributor::new())
    }

    pub fn to_contributor(&self) -> Option<&Contributor> {
        match self {
            User::Contributor(contributor) => Some(contributor),
            _ => None
        }
    }

    pub fn to_contributor_mut(&mut self) -> Option<&mut Contributor> {
        match self {
            User::Contributor(contributor) => Some(contributor),
            _ => None
        }
    }

    pub fn to_pensioner(&mut self) -> Option<&mut Pensioner> {
        match self {
            User::Pensioner(pensioner) => Some(pensioner),
            _ => None
        }
    }

    pub fn to_done_user(&mut self) -> Option<&mut DoneUser> {
        match self {
            User::Done(done_user) => Some(done_user),
            _ => None
        }
    }
}