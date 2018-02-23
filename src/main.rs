extern crate irc_client;

use irc_client::chat_stream::tcp_stream::TcpStreamWrapper;
use irc_client::chat_stream::ChatStream;
use irc_client::config::*;

use irc_client::irc_connection::*;
use std::io::*;

fn main() {
    let config = get_config();
   
    let mut stream = TcpStreamWrapper::new();
    let mut irc = IRC::new(&mut stream);

    irc.send_raw(&format!("NICK {}", config.name));
    irc.send_raw(&format!("USER {0} 0 * :{0} {0}", config.name));

   irc.read(|ref _client, &dat| {
        let text = String::from_utf8_lossy(&dat);
        println!("{}", text);
        Ok(())
   });
    

}

fn get_config() -> Config {
    let config;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "-i" {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer).expect("invalid config");
        config = get_config_from_string(buffer.as_str()).expect("invalid config")
    }else{
        config = default_config();
    }
    config
}

