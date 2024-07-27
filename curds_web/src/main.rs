mod server;

use server::CurdsWebServer;
use tokio::sync::oneshot::channel;
use tokio::signal::ctrl_c;

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
