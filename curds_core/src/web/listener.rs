
pub trait TcpListenerFactory {
    type Temp : std::io::Read;
}