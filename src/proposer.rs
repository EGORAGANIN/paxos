use crate::messages::*;

pub struct Proposer<'a, U: Messenger> {
    id: u32,
    proposal_id: u32,
    proposal_value: Option<i32>,
    quorum_size: u32,
    controller: &'a U,
    quorum_count: u32,
}

impl<'a, U: Messenger> Proposer<'a, U> {

    pub fn new(id: u32, quorum_size: u32, controller: &U) -> Proposer<U> {
        Proposer {
            id,
            proposal_id: 1,
            proposal_value: None,
            quorum_size,
            controller,
            quorum_count: 0,
        }
    }

    pub fn prepare(&mut self, value: i32) {
        let proposal = Proposal::new(self.proposal_id, value);
        self.proposal_value = Some(proposal.value);
        self.proposal_id += 1;
        self.quorum_count = 0;

        self.controller.send_prepare(Message::Prepare(proposal));
    }

    pub fn process_promise(&mut self, message: Message) {
        if let Message::Promise { promise, from } = message {
            if self.proposal_id != from.id { return; }

            if self.proposal_id < promise.id {
                self.proposal_id = promise.id;
                self.proposal_value = Some(promise.value);
            }
            self.quorum_count += 1;

            if self.quorum_size == self.quorum_count {

                self.controller.send_accept(
                    Message::Accept(
                        Proposal::new(self.proposal_id, self.proposal_value.unwrap())
                    )
                )
            }
        }
    }
}
