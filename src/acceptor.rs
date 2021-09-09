use crate::messages::*;

pub struct Acceptor<'a, U: Messenger> {
    id: u32,
    promised_proposal: Option<Proposal>,
    accept: Option<Proposal>,
    controller: &'a U,
}

impl<'a, U: Messenger> Acceptor<'a, U> {

    fn new(id: u32, controller: &U) -> Acceptor<U> {
        Acceptor {
            id,
            promised_proposal: None,
            accept: None,
            controller,
        }
    }

    fn process_prepare(&mut self, message: Message) {
        if let Message::Prepare(proposal) = message {

            let promise: Message = match self.promised_proposal {

                Some(current_proposal) => {
                    if proposal.id > current_proposal.id {
                        self.promised_proposal = Some(proposal);
                        Message::Promise {
                            promise: proposal,
                            from: proposal,
                        }
                    } else {
                        Message::Promise {
                            promise: current_proposal,
                            from: proposal,
                        }
                    }
                }
                None => {
                    self.promised_proposal = Some(proposal);
                    Message::Promise {
                        promise: proposal,
                        from: proposal,
                    }
                }
            };

            self.controller.send_promise(promise);
        }
    }

    fn process_accept(&mut self, message: Message) {
        if let Message::Accept(proposal) = message {
            if let Some(self_promised_proposal) = &self.promised_proposal {
                if proposal.id >= self_promised_proposal.id {
                    self.promised_proposal = Some(proposal);
                    self.accept = Some(proposal);
                    self.controller.send_accepted(Message::Accepted(proposal));
                }
            }
        }
    }
}