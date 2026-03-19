use std::rc::Rc;
use std::thread::spawn;

use smol::channel::{Receiver, RecvError, Sender, bounded, unbounded};
use smol::{block_on, LocalExecutor};

use crate::io::comm::{IoMessage, IoResponse};
use crate::io::vts::connect_vts_tracker;
use crate::io::error::Reportable;

/// Thread process for non-window-system I/O.
fn io_main(recv: Receiver<IoMessage>, send: Sender<IoResponse>) {
    let ex = Rc::new(LocalExecutor::new());
    let (shutdown_send, shutdown_recv) = bounded(1);

    let inner_ex = ex.clone();

    ex.spawn(async move {
        loop {
            let inner_send = send.clone();
            match recv.recv().await {
                Ok(IoMessage::Exit) => {
                    let inner_shutdown_send = shutdown_send.clone();
                    inner_ex.spawn((async move || {
                        inner_shutdown_send.send(()).await.report(inner_send).await;
                    })()).detach()
                },
                Ok(IoMessage::ConnectVTSTracker(addr)) => {
                    let vts_ex = inner_ex.clone();
                    inner_ex.spawn((async move || {
                        connect_vts_tracker(vts_ex, addr).await.report(inner_send).await;
                    })()).detach();
                },
                Err(e) => {
                    Err::<(), RecvError>(e).report(inner_send).await;
                }
            }
        }
    }).detach();

    block_on(shutdown_recv.recv()).unwrap();
}

/// Spawn the IO thread.
/// 
/// This function returns channels that can be used to make asynchronous
/// requests on the IO thread. You do not need to actually be in async-colored
/// functions in order to use them, they work like std's MPSC channels.
pub fn start() -> (Sender<IoMessage>, Receiver<IoResponse>) {
    let (message_send, message_recv) = unbounded();
    let (response_send, response_recv) = unbounded();

    spawn(|| {
        io_main(message_recv, response_send)
    });

    (message_send, response_recv)
}