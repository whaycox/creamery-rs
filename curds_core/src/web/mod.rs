mod request;
mod method;
mod version;
mod headers;
mod response;
mod status;
mod parsing;
mod error;
mod listener;

pub use request::*;
pub use method::*;
pub use version::*;
pub use headers::*;
pub use response::*;
pub use status::*;
pub use parsing::*;
pub use error::*;
pub use listener::*;

pub type CurdsWebResult<T> = Result<T, CurdsWebError>;

#[cfg(feature = "test-helpers")]
use tokio::net::{TcpStream, TcpListener};
#[cfg(feature = "test-helpers")]
use std::net::SocketAddr;

#[cfg(feature = "test-helpers")]
pub async fn test_connection(address: &str) -> (SocketAddr, TcpStream, TcpStream) {
    let listener = TcpListener::bind(address).await.unwrap();
    let socket = listener.local_addr().unwrap();
    
    let handle = tokio::spawn(async move {
        let stream = listener.accept().await.unwrap();
        stream.0
    });

    let client = TcpStream::connect(socket).await.unwrap();
    let server = handle.await.unwrap();

    (socket, client, server)
}