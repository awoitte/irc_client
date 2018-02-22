#[macro_use]
extern crate json;

use std::io::*;
use std::net::TcpStream;
use std::string;

fn main() {
    //let config = get_config();
    //println!("{}", config.unwrap());
    
    let mut stream = TcpStream::connect("irc.mozilla.org:6667").expect("error connecting");

    //stream.set_read_timeout(Some(std::time::Duration::from_secs(4))).expect("error setting timeout");

    //send_raw("CAP LS 302", &stream);
    send_raw("NICK bamboo", &stream);
    send_raw("USER bamboo 0 * :bamboo bamboo", &stream);
    // send_raw("MOTD", &stream);
    //send_raw("JOIN #newbies", &stream);
    //send_raw("CAP END", &stream);

   read(stream, |&dat|{
        let text = String::from_utf8_lossy(&dat);
        println!("{}", text);
        Ok(())
   });
    

}

fn read<F>(mut stream: TcpStream, respond: F)
    where F: Fn(&[u8; 512])-> Result<()> {

    let do_loop = true;
    while do_loop {
        let mut dat = [0; 512];
        match stream.read(&mut dat){
            Ok(n) => {
                if n > 0 {
                    respond(&dat);
                }
            },
            _ => {}
        }
    }
    send_raw("QUIT", &stream);
}

fn send_message(chan:&str, message:&str, mut stream:&TcpStream ){
    let command = format!("PRIVMSG {} {}", chan, message);
    send_raw(&command, stream);
}

fn send_raw(message:&str, mut stream:&TcpStream ){
    let final_string = format!("{}\r\n", message);
    print!("{}", final_string);
    
    let wrote = stream.write(final_string.as_bytes()).expect("error writing");

    println!("wrote {} bytes", wrote);
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


