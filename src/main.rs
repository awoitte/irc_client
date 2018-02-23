//#[macro_use]
extern crate json;

use std::io::*;
extern crate irc_client;

use irc_client::chat_stream::tcp_stream::TcpStreamWrapper;
use irc_client::chat_stream::ChatStream;

use irc_client::irc_connection::*;

fn main() {
    //let config = get_config();
    //println!("{}", config.unwrap());
    let mut stream = TcpStreamWrapper::new();
    let mut irc = IRC::new(&mut stream);
    //stream.set_read_timeout(Some(std::time::Duration::from_secs(4))).expect("error setting timeout");

    //send_raw("CAP LS 302", &stream);
    irc.send_raw("NICK bamboo");
    irc.send_raw("USER bamboo 0 * :bamboo bamboo");
    // send_raw("MOTD", &stream);
    //send_raw("JOIN #newbies", &stream);
    //send_raw("CAP END", &stream);

   irc.read(|&dat| {
        let text = String::from_utf8_lossy(&dat);
        println!("{}", text);
        Ok(())
   });
    

}


//reads json from stdin
fn get_config () -> Result<json::JsonValue>{
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;
    match json::parse(buffer.as_str()){
        Ok(result) => Ok(result),
        Err(error) => {
            Err(Error::new(ErrorKind::InvalidInput, error))
        },
    }
}


