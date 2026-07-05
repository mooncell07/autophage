use crate::core::errors::BackendError;
use crate::core::models::Signals;

use super::core::session::Session;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::async_runtime::spawn_blocking;
use tauri::State;
//use tokio::time::{sleep, Duration};

macro_rules! _get_session_guard {
    ($session:expr) => {
        $session.lock().map_err(|e| {
            BackendError::Other(
                format!("Failed to gain access to session guard: {}", e.to_string()).into(),
            )
        })
    };
}

#[tauri::command]
pub async fn start_session(
    session: State<'_, Arc<Mutex<Session>>>,
    filepath: PathBuf,
) -> Result<String, BackendError> {
    let session_clone = Arc::clone(&session);

    let _ = spawn_blocking(move || {
        let mut guard = _get_session_guard!(session_clone)?;
        guard.init(filepath)
    })
    .await??;
    Ok(Signals::SessionReady.to_string())
}

#[tauri::command]
pub async fn analyze_binary(
    session: State<'_, Arc<Mutex<Session>>>,
) -> Result<String, BackendError> {
    let session_clone = Arc::clone(&session);
    //sleep(Duration::from_secs(5)).await;
    let _ = spawn_blocking(move || {
        let mut guard = _get_session_guard!(session_clone)?;
        guard.submit("aaa")
    })
    .await??;
    Ok(Signals::BinaryAnalyzed.to_string())
}

#[tauri::command]
pub async fn get_information(
    session: State<'_, Arc<Mutex<Session>>>,
) -> Result<String, BackendError> {
    let mut guard = _get_session_guard!(session)?;
    let result = guard.submit("ij")?;
    Ok(result)
}

#[tauri::command]
pub async fn get_functions(
    session: State<'_, Arc<Mutex<Session>>>,
) -> Result<String, BackendError> {
    let mut guard = _get_session_guard!(session)?;
    let result = guard.submit("aflj")?;
    Ok(result)
}

#[tauri::command]
pub fn close_application(session: State<'_, Arc<Mutex<Session>>>) -> Result<String, BackendError> {
    let mut guard = _get_session_guard!(session)?;
    guard.close();
    Ok(Signals::SessionClosed.to_string())
}
