use crate::config::{init_config, update_config};
use http_body_util::{BodyExt, Empty};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, UnixListener};
use tokio::sync::broadcast::Sender;

pub async fn start(
    shutdown_tx: Sender<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (port, host) = {
        let config = init_config().await.read_owned().await;
        (config.port(), config.host())
    };
    let listener = TcpListener::bind(format!("{host}:{port}")).await?;

    println!("Listening on http://{host}:{port}");

    let mut shutdown_rx = shutdown_tx.subscribe();

    let connections = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    loop {
        tokio::select! {
            accept_result = listener.accept() => {
                let (stream, _) = accept_result?;

                let io = TokioIo::new(stream);
                let mut conn_shutdown_rx = shutdown_tx.subscribe();
                let connections = connections.clone();

                let task = tokio::spawn(async move {

                    let connection = http1::Builder::new()
                        .serve_connection(io, service_fn(handle));

                    tokio::select! {
                        result = connection => {
                            if let Err(err) = result {
                                eprintln!("Connection error: {:?}", err);
                            }
                        }

                        _ = conn_shutdown_rx.recv() => {
                            println!("Connection received shutdown signal");
                        }
                    }
                });

                connections.lock().await.push(task);
            }

            _ = shutdown_rx.recv() => {
                println!("Stopping server...");
                break;
            }
        }
    }

    println!("Waiting for active connections to finish...");

    let mut tasks = connections.lock().await;

    while let Some(task) = tasks.pop() {
        let _ = task.await;
    }

    println!("Server stopped");

    Ok(())
}

async fn handle(req: Request<Incoming>) -> Result<Response<Empty<Bytes>>, Infallible> {
    if req.method().ne(&Method::POST) {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Empty::new())
            .unwrap());
    }

    let (parts, body) = req.into_parts();

    let method = match init_config()
        .await
        .read()
        .await
        .method(parts.uri.path().chars().skip(1).collect::<String>())
        .cloned()
    {
        Some(method) => method,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Empty::new())
                .unwrap());
        }
    };

    let mut args: Vec<String> = Vec::new();

    if let Some(query) = parts.uri.query() {
        let params = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .filter_map(|(k, v)| k.parse::<i32>().ok().map(|k| (k, v)))
            .collect::<HashMap<i32, String>>();

        for i in 1.. {
            if let Some(arg) = params.get(&i) {
                args.push(arg.clone());
            } else {
                break;
            }
        }
    }

    let bytes = match body.collect().await {
        Ok(stdin) => {
            let bytes = stdin.to_bytes();
            if bytes.is_empty() { None } else { Some(bytes) }
        }
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Empty::new())
                .unwrap());
        }
    };

    let _ = method.execute(args, bytes).await;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Empty::new())
        .unwrap())
}

pub fn send_stop_message_to_unix_socket() {
    match std::os::unix::net::UnixStream::connect("/tmp/swhook.sock") {
        Ok(mut stream) => {
            let _ = std::io::Write::write_all(&mut stream, b"stop");
        }
        Err(err) => eprintln!("Error: {err}"),
    }
}

pub fn send_reload_message_to_unix_socket() {
    match std::os::unix::net::UnixStream::connect("/tmp/swhook.sock") {
        Ok(mut stream) => {
            let _ = std::io::Write::write_all(&mut stream, b"reload");
        }
        Err(err) => eprintln!("Error: {err}"),
    }
}

pub async fn start_unix_socket_listener(shutdown_tx: Sender<()>) -> std::io::Result<()> {
    let path = "/tmp/swhook.sock";
    let _ = tokio::fs::remove_file(path).await;

    let listener = UnixListener::bind("/tmp/swhook.sock")?;

    println!("Listening to unix socket [/tmp/swhook.sock]...");

    loop {
        let (stream, _) = listener.accept().await?;
        let server_shutdown = shutdown_tx.clone();

        tokio::spawn(async move {
            let mut reader = BufReader::new(stream);
            let mut line = String::new();

            reader.read_line(&mut line).await.unwrap_or_default();

            if line.trim() == "stop" {
                let _ = server_shutdown.send(());
            } else if line.trim() == "reload" {
                update_config().await;
            }
        });
    }
}
