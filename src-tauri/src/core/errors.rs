use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Rizin Error: {0}")]
    Rizin(String),

    #[error("Command failed to execute: {0}")]
    Command(String),

    #[error("Tauri Error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("Error: {0}")]
    Other(String),
}

impl serde::Serialize for BackendError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
