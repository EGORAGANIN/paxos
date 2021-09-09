use crate::messages::{Proposal, Message};

pub struct Learner {
    accepted_value: Option<Proposal>
}

impl Learner {

    fn new() -> Learner {
        Learner {
            accepted_value: None
        }
    }

    fn process_accepted(& mut self, message: Message) {
        if let Message::Accepted(proposal) = message {
            self.accepted_value = Some(proposal);
        }
    }
}
