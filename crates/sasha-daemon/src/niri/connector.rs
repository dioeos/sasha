use std::{ffi::OsString, path::PathBuf, env::var_os};
use anyhow::{Context, Result, anyhow};
use tokio::net::UnixStream;

pub struct NiriConnector {
    socket_path: PathBuf
}

impl NiriConnector {
    pub fn new_from_env() -> Result<Self>  {
        let niri_socket_path: OsString = var_os("NIRI_SOCKET")
            .ok_or_else(|| anyhow!("Cannot find NIRI_SOCKET var"))?;

        Ok(Self {
            socket_path: PathBuf::from(niri_socket_path)
        })
    }

    pub async fn connect(&self) -> Result<UnixStream> {
        UnixStream::connect(&self.socket_path)
            .await
            .with_context(|| format!(
                    "Failed to connect to Niri socket at {}",
                    self.socket_path.display()
            ))
    }
}
