mod server;
mod responder;

use server::CurdsWebServer;
use responder::*;
use tokio::sync::oneshot::channel;
use tokio::signal::ctrl_c;
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use curds_core::{io::AsyncFileSystem, web::{CurdsWebError, CurdsWebHttpRequestParser, HttpMethod, HttpRequest, HttpRequestParser, HttpResponse, HttpStatus, HttpVersion}};
use tokio::io::BufReader;
use std::{net::SocketAddr, pin::Pin};
use std::future::Future;
use std::sync::Arc;
use curds_core::web::TestingHttpRequestParser;

#[tokio::main]
async fn main() {
    curds_core::logger::initialize();

    let server = CurdsWebServer::new();
    let (sender, receiver) = channel::<()>();
    tokio::spawn(async move {
        ctrl_c().await.expect("Failed to listen for ctrl+c");
        sender.send(()).expect("Failed to notify of closing");
    });

    tokio::select! {
        _ = server.start() => {},
        _ = receiver => { log::info!("Closing down"); },
    };
}