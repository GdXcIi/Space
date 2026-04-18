use std::collections::HashMap;

pub struct IPCManager {
    channels: HashMap<ModuleID, Vec<IPCMessage>>,
    permissions: HashMap<ModuleID, Vec<ModuleID>>,
}

impl IPCManager {
    /// Send a message IF permissions are ok
    pub fn send(
        &mut self,
        sender: ModuleID,
        target: ModuleID,
        payload: Vec<u8>,
        ticket_id: TicketID,
    ) -> Result<(), String> {
        if !self
            .permissions
            .get(&sender)
            .map_or(false, |allowed| allowed.contains(&target))
        {
            return Err("Permission denied".to_string());
        }
        self.channels
            .entry(target)
            .or_default()
            .push(IPCMessage { sender, payload });
        Ok(())
    }

    /// Receive a message for a module
    fn receive(&mut self, module_id: ModuleID) -> Option<IPCMessage> {
        self.channels.get_mut(&module_id).and_then(|q| q.pop())
    }
}

pub struct IPCMessage {
    sender: ModuleID,
    payload: Vec<u8>,
}
