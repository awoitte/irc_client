extern crate irc_client;
use std::io::*;
use irc_client::irc_connection::IRC;
use irc_client::chat_stream::ChatStream;



pub struct MockStream {
    last_write: String,
    read_error: bool
}

impl irc_client::chat_stream::ChatStream for MockStream {
    fn new() -> Self{
        MockStream {
            last_write: String::new(),
            read_error: false
        }
    }

    fn read(&mut self, mut data: &mut[u8]) -> Result<usize>{
        if self.read_error {
            Err(std::io::Error::last_os_error())
        }
        else{
            Ok(512)
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize>{
        let write = String::from_utf8_lossy(buf);
        self.last_write = format!("{}", write);
        Ok(512)
    }

}

impl MockStream {
    fn get_last_write(&self)->String{
        self.last_write.to_owned()
    }

    fn stop_reading(&mut self) -> (){
        self.read_error = true;
    }
}

#[test]
fn it_should_attach_line_breaks_to_raw(){
    let mut stream = MockStream::new();
    let mut irc = IRC::new(&mut stream);
    irc.send_raw("test");
    assert_eq!(irc.stream.get_last_write(), "test\r\n");
}

