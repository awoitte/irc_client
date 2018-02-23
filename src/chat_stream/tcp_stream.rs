use std::net::TcpStream;
use std::io::*;
use chat_stream::ChatStream;

pub struct TcpStreamWrapper {
    stream: TcpStream
}

impl ChatStream for TcpStreamWrapper {
    fn new() -> Self{
        TcpStreamWrapper{
            stream: TcpStream::connect("irc.mozilla.org:6667").expect("error connecting")
        }
    }

    fn read(&mut self, mut data: &mut[u8]) -> Result<usize>{
        self.stream.read(&mut data)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize>{
        self.stream.write(buf)
    }
}
