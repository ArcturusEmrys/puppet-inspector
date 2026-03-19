//! Asynchronous I/O thread

use std::io;
use std::str::Utf8Error;

use smol::channel::{RecvError, Sender, SendError};
use thiserror::Error;

use crate::io::comm::IoResponse;

#[derive(Error, Debug)]
pub enum IoThreadError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("UTF-8 parsing error: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("Json parsing error: {0}")]
    Json(#[from] json::Error),
    #[error("Channel recieve error: {0}")]
    Recv(#[from] RecvError),
    #[error("Channel send error")]
    SendIoResponse,
    #[error("Channel send error (void/shutdown)")]
    SendNothing,
    
}

impl From<SendError<IoResponse>> for IoThreadError {
    fn from(_err: SendError<IoResponse>) -> Self {
        Self::SendIoResponse
    }
}

impl From<SendError<()>> for IoThreadError {
    fn from(_err: SendError<()>) -> Self {
        Self::SendNothing
    }
}

pub trait Reportable {
    async fn report(self, send: Sender<IoResponse>);
}

impl<T, E> Reportable for Result<T, E> where E: Into<IoThreadError> {
    async fn report(self, send: Sender<IoResponse>) {
        match self {
            Ok(_) => {},
            Err(e) => {
                // Intentionally ignored failure: if this send fails that
                // means the main thread has already gone away and we should
                // go away too
                let _ = send.send(IoResponse::Error(e.into())).await;
            }
        }
    }
}