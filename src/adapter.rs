// An Adapter for wrapping the inner `BrideClient` for the purpose of handling a Session.

use crate::models::FunctionList;

use super::models::Disassembly;
use anyhow;
use ghidra_cli::config::Config;
use ghidra_cli::ghidra::bridge::{self, BridgeStartMode};
use ghidra_cli::ipc::client::BridgeClient;

pub struct Adapter {
    config: Config,
    bridge_client: BridgeClient,
}

impl Adapter {
    pub fn new(project_name: &str) -> anyhow::Result<Self> {
        let config = Config::load()?;
        let project_dir = config.get_project_dir()?;
        let project_path = project_dir.join(project_name);

        let ghidra_install_dir = config.get_ghidra_install_dir()?;

        let port = if let Some(port) = bridge::is_bridge_running(&project_path) {
            port
        } else {
            bridge::ensure_bridge_running(
                &project_path,
                &ghidra_install_dir,
                BridgeStartMode::Project,
            )?
        };

        let bridge_client = BridgeClient::new(port);

        Ok(Self {
            config,
            bridge_client,
        })
    }

    pub fn list_functions(&self) -> anyhow::Result<FunctionList> {
        let res = self.bridge_client.list_functions(None, None)?;
        Ok(serde_json::from_value::<FunctionList>(res).unwrap())
    }

    pub fn list_strings(&self) -> anyhow::Result<serde_json::Value> {
        self.bridge_client.list_strings(None, None)
    }

    pub fn list_imports(&self) -> anyhow::Result<serde_json::Value> {
        self.bridge_client.list_imports()
    }
    pub fn list_exports(&self) -> anyhow::Result<serde_json::Value> {
        self.bridge_client.list_exports()
    }

    pub fn list_programs(&self) -> anyhow::Result<serde_json::Value> {
        self.bridge_client.list_programs()
    }

    pub fn get_program_info(&self) -> anyhow::Result<serde_json::Value> {
        self.bridge_client.program_info()
    }

    pub fn get_disassembly(&self, address: &str, instr: usize) -> anyhow::Result<Disassembly> {
        let res = self.bridge_client.disasm(address, Some(instr))?;
        Ok(serde_json::from_value::<Disassembly>(res).unwrap())
    }

    pub fn close(&self) {
        let _ = self.bridge_client.shutdown();
    }
}
