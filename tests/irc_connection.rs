extern crate irc_client;
use std::io::*;
use irc_client::irc_connection::IRC;
use irc_client::chat_stream::ChatStream;



pub struct MockStream {
    last_write: String,
    reads_count: i32 
}

impl irc_client::chat_stream::ChatStream for MockStream {
    fn new() -> Self{
        MockStream {
            last_write: String::new(),
            reads_count: 0
        }
    }

    fn read(&mut self, _: &mut[u8]) -> Result<usize>{
        self.reads_count += 1;
        Ok(512)
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
}

#[test]
fn it_should_attach_line_breaks_to_raw(){
    let mut stream = MockStream::new();
    let mut irc = IRC::new(&mut stream);
    irc.send_raw("test");
    assert_eq!(irc.stream.get_last_write(), "test\r\n");
}


#[test]
fn it_should_add_channel_and_privmsg(){
    let mut stream = MockStream::new();
    let mut irc = IRC::new(&mut stream);
    irc.send_message("chan", "test");
    assert_eq!(irc.stream.get_last_write(), "PRIVMSG chan test\r\n");
}

#[test]
fn it_should_stop_reading_on_error(){
    let mut stream = MockStream::new();
    let mut irc = IRC::new(&mut stream);
    irc.read(|ref client, _| {
        if client.stream.reads_count == 1 {
            Ok(())
        }else{
            Err(Error::last_os_error())
        }
    });
    assert_eq!(irc.stream.reads_count, 2);
}
