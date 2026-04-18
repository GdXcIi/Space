use shared::Permission;
use toml::Value;

pub struct SecurityManager {
    manifests: HashMap<ModuleID, Value>, // Loaded from manifest.toml
}

impl SecurityManager {
    pub fn check_ipc_permission(&self, caller: ModuleID, target: ModuleID) -> bool {
        self.manifests
            .get(&caller)
            .and_then(|m| m.get("allowed_ipc_targets"))
            .and_then(|targets| targets.as_array())
            .map_or(false, |targets| {
                targets
                    .iter()
                    .any(|t| t.as_str() == Some(&target.to_string()))
            })
    }
}
