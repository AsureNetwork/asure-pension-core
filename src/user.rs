use crate::contributor::Contributor;
use crate::pensioner::Pensioner;
use crate::doneuser::DoneUser;

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

    pub fn to_pensioner(&self) -> Option<&Pensioner> {
        match self {
            User::Pensioner(pensioner) => Some(pensioner),
            _ => None
        }
    }

    pub fn to_pensioner_mut(&mut self) -> Option<&mut Pensioner> {
        match self {
            User::Pensioner(pensioner) => Some(pensioner),
            _ => None
        }
    }

    pub fn to_done_user(&self) -> Option<&DoneUser> {
        match self {
            User::Done(done_user) => Some(done_user),
            _ => None
        }
    }
}