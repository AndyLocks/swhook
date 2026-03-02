use crate::server::{start, start_unix_socket_listener};
use tokio::runtime::Runtime;
use tokio::sync::broadcast;

pub fn server() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (shutdown_tx, _) = broadcast::channel::<()>(16);

        let server_shutdown = shutdown_tx.clone();
        let unix_socket_listener_shutdown = shutdown_tx.clone();
        let mut conn_shutdown_rx = shutdown_tx.subscribe();

        let server_task = tokio::spawn(async move { start(server_shutdown).await });

        let _ =
            tokio::spawn(
                async move { start_unix_socket_listener(unix_socket_listener_shutdown).await },
            );

        tokio::select! {
            _ = async {tokio::signal::ctrl_c().await} => {
                println!("Shutdown signal received");
                let _ = shutdown_tx.send(());
                if let Err(err) = server_task.await {
                    eprintln!("Error: {err}")
                }
            },
            _ = conn_shutdown_rx.recv() => {},
        }
    });
}
