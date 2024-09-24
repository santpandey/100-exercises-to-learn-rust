use std::sync::mpsc::{Receiver, Sender};
use data::{Ticket, TicketDraft};
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { draft: TicketDraft,  resp: Sender<TicketId> },
    Get { id: TicketId, resp: Sender<Option<Ticket>> }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {draft, resp}) => {
                let ticket_id = store.add_ticket(draft);
                let _ = resp.send(ticket_id);
            }
            Ok(Command::Get {
                id, resp
            }) => {
                let ticket = store.get(id);
                let _ = resp.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
