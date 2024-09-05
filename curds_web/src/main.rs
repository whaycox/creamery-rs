mod server;
mod responder;
mod file_router;
mod route_map;
mod site_configuration;

use server::CurdsWebServer;
use responder::*;
use file_router::*;
use route_map::*;
use site_configuration::*;
use tokio::sync::oneshot::channel;
use tokio::signal::ctrl_c;
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use curds_core::{io::AsyncFileSystem, web::{CurdsWebError, CurdsWebHttpRequestParser, HttpMethod, HttpRequest, HttpRequestParser, HttpResponse, HttpStatus, HttpVersion}};
use tokio::io::BufReader;
use std::{net::SocketAddr, pin::Pin};
use std::future::Future;
use curds_core::web::CurdsWebResult;
use std::sync::Arc;
use curds_core::web::TestingHttpRequestParser;
use std::collections::HashMap;
use regex::Regex;
use curds_core::{io::FileSystem, whey_mock, web::UriPath};
use std::{collections::hash_map::Entry, path::Path};
use tokio::fs::DirEntry;
use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventHandler, DebounceEventResult, Debouncer, FileIdMap};
use notify::event::{ModifyKind, RenameMode};
use std::time::Duration;
use std::sync::RwLock;

#[tokio::main]
async fn main() {
    curds_core::logger::initialize();

    let server = CurdsWebServer::new().await.unwrap();
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