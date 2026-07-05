use std::path::PathBuf;

use rzpipe::RzPipe;
use rzpipe::RzPipeSpawnOptions;

use crate::core::errors::BackendError;

pub struct Session {
    pub exepath: String,
    pub rzp: Option<RzPipe>,
}
impl Session {
    pub fn preinit() -> Self {
        Self {
            exepath: "rizin".to_string(),
            rzp: None,
        }
    }

    pub fn init(&mut self, filepath: PathBuf) -> Result<(), BackendError> {
        let opts = RzPipeSpawnOptions {
            exepath: self.exepath.clone(),
            ..Default::default()
        };
        self.rzp = Some(
            RzPipe::spawn(filepath.to_string_lossy(), Some(opts)).map_err(|e| {
                BackendError::Rizin(format!(
                    "Child Process `{}` failed to spawn: {}",
                    self.exepath,
                    e.to_string()
                ))
            })?,
        );
        Ok(())
    }

    pub fn submit(&mut self, command: &str) -> Result<String, BackendError> {
        let rzp = self
            .rzp
            .as_mut()
            .ok_or(BackendError::Rizin("Rizin Instance Uninitiated".into()))?;
        let result = rzp
            .cmd(command)
            .map_err(|e| BackendError::Command(e.to_string()))?;
        Ok(result.to_string())
    }

    pub fn close(&mut self) {
        match &mut self.rzp {
            Some(r) => r.close(),
            None => (),
        };
    }
}
