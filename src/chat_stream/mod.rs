pub mod tcp_stream;

use std::io::*;

pub trait ChatStream{
    fn read(&mut self, data: &mut[u8]) -> Result<usize>;
    fn new() -> Self;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}
