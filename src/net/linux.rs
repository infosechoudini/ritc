
pub struct SockAddrInet4 {
    Port: int,
    Addr: Vec<u8>,
    raw: RawSockaddrInet4,
}

pub struct RawSockaddrInet4 {
    Family: u16,
    Port: u16,
    Addr: Vec<u8>,
    Zero: Vec<u8>,
}
