// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError, channel, sync_channel};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadError> {
        let (request, response) = channel();
        self.sender.try_send(Command::Insert {
             draft, response_channel: request,
            })?;
        Ok(response.recv().expect("Failed to receive response"))
           
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadError> {
        let (request, response) = channel();
        self.sender.try_send(Command::Get { 
            id, response_channel: request, })?;
        Ok(response.recv().expect("Failed to send response"))    
            
    }
}

#[derive(Debug, Clone)]
pub struct OverloadError{
}

impl <T> From<TrySendError<T>> for OverloadError {
    fn from(_: TrySendError<T>) -> Self {
         OverloadError{}
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {

    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned());          }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
