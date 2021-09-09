use crate::client::Client;
use crate::proposer::Proposer;

pub enum Message {
    Prepare(Proposal),
    Promise { promise: Proposal, from: Proposal },
    Accept(Proposal),
    Accepted(Proposal),
}

#[derive(Copy, Clone)]
pub struct Proposal {
    pub id: u32,
    pub value: i32,
}

impl Proposal {
    pub fn new(id: u32, value: i32) -> Proposal {
        Proposal { id, value }
    }
}

pub trait Messenger {
    fn send_prepare(&self, message: Message);

    fn send_promise(&self, message: Message);

    fn send_accept(&self, message: Message);

    fn send_accepted(&self, message: Message);
}