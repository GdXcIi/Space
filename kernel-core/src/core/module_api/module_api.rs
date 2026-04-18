use serde::{Deserialize, Serialize};
use shared::{FID, ModuleID};
use std::collections::HashMap;
use toml::Value;

enum IPCMessage {
    Request { sender: ModuleID, payload: Vec<u8> },
    Response { success: bool, payload: Vec<u8> },
}

#[derive(Debug, Clone)]
pub enum KernelRequest {
    FS(FSRequest),
    UI(UIRequest),
    Auth(AuthRequest),
    Input(InputRequest),
    Network(NetworkRequest),
    Notify(NotifyRequest),
    Resolver(ResolverRequest),
}

#[derive(Debug, Clone)]
pub enum FSRequest {
    CreateNode(u8, &[u8]),
    ReadNode(FID),
    WriteNode(FID, Vec<u8>),
    DeleteNode(FID),
    LinkNode(FID, FID),
}

#[derive(Debug, Clone)]
pub enum UIRequest {
    CreateWindow(&[u8]),
    RenderBuffer(WindowID, &[u8]),
    CloseWindow(WidowID),
    OnEvent(Box<dyn FnMut(&[u8])>),
}

#[derive(Debug, Clone)]
pub enum AuthRequest {
    Login(&[u8]),
    Logout(),
    CheckPermission(&[u8]),
}

#[derive(Debug, Clone)]
pub enum InputRequest {
    Subscribe(),
    PollEvents(),
    OnEvent(Box<dyn FnMut(&[u8])>),
}

#[derive(Debug, Clone)]
pub enum NetworkRequest {
    OpenSocket(u8),
    Send(SocketID, &[u8]),
    Receive(SocketID),
    CloseSocket(SocketID),
}

#[derive(Debug, Clone)]
pub enum NotifyRequest {
    SendNotification(UserID, &[u8], u8),
}

#[derive(Debug, Clone)]
pub enum ResolverRequest {
    Resolve(&str),
}

// -- API exposed to modules --
pub trait ModuleAPI {
    /// Send a request to kernel (serialized in binary)
    fn kernel_request(&mut self, request: &[u8]) -> Result<Vec<u8>, String>;

    /// Receive a response from kernel (deserialized for the module)
    fn kernel_response(&mut self) -> Option<Vec<u8>>;
}

// -- Kernel-side implementation --
pub struct KernelModuleAPI {
    ipc: IPCManager,
    security: SecurityManager,
    module_id: ModuleID,
}

impl KernelModuleAPI {
    pub fn new(ipc: IPCManager, security: SecurityManager, module_id: ModuleID) -> Self {
        Self {
            ipc,
            security,
            module_id,
        }
    }
}

impl ModuleAPI for KernelModuleAPI {
    fn kernel_request(&mut self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // 1. Deserialize request
        let request: KernelRequest =
            bincode::deserialize(request_bytes).map_error(|e| format!("Invalid request: {}", e))?;

        // 2. Check permissions (example: for the FS)
        match &request {
            KernelRequest::FS(FSRequest::ReadNode(fid)) => {
                if !self.security.check_permission(self.module_id, fs.read) {
                    return Err("Permission denied: fs.read".to_string());
                }
                // 3. Transmit to FS module (via internal IPC)
                let fs_response = self.ipc.send_to_fs_module(FSRequest::ReadNode(*fid))?;
                Ok(bincode::serialize(&fs_response).unwrap())
            }
        }
    }
}
