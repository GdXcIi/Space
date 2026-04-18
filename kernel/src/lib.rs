use module_api::IPCMessage;
use std::collections::HashMap;



/// === Global variables ===

type FID = u64;
type ModuleID = u32;
type Timestamp = u64;
type RandomNumber = u32;
type TicketID = HashMap<RandomNumber, ModuleID, Timestamp>;




/// === IPC ===

struct IPCManager {
    chennels: HashMap<FID, Vec<IPCMessage>>,
}

impl IPCManager {
    pub fn send(&mut self, receiver: FID, message: IPCMessage) {
        self.channels.entry(receiver).or_default().push(message);
    }

    pub fn receive(&mut self, fid: FID) -> Option<IPCMessage> {
        self.channels.get_mut(&fid).and_then(|q| q.pop())
    }
}


/// === Tickets ===

struct Ticket {
    id: TicketID,
    module_from: u32,
    module_to: ModuleID,
    permissions: Vec<u32>,
    expiration: u64,
    version: u32,
    signature: &str,
}
