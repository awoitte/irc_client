use std::io::*;

use chat_stream::*;

pub struct IRC<'a, T>
    where T:'a + ChatStream {
    pub stream: &'a mut T // TODO: probably shouldn't be public, for ease of testing
}

impl<'a, T:ChatStream> IRC<'a, T>{
    pub fn new(stream: &'a mut T) -> Self{
        IRC{
            stream
        }
    }

    pub fn read<F>(&mut self, respond: F) //TODO: Async
        where F: Fn(&IRC<'a, T>, &[u8; 512])-> Result<()> {

        let mut do_loop = true;
        while do_loop {
            let mut dat = [0; 512];
            match self.stream.read(&mut dat){
                Ok(n) => {
                    if n > 0 {
                        match respond(self, &dat){
                            Err(_)=>do_loop = false ,
                            _ => ()
                        }
                    }
                },
                Err(error) => println!("error: {}", error) 
            }
        }
        self.send_raw("QUIT");
    }

    pub fn send_message(&mut self, chan:&str, message:&str){
        let command = format!("PRIVMSG {} {}", chan, message);
        self.send_raw(&command);
    }

    pub fn send_raw(&mut self, message:&str){
        let final_string = format!("{}\r\n", message);
        print!("{}", final_string);
        
        let wrote = self.stream.write(final_string.as_bytes()).expect("error writing");
        println!("wrote {} bytes", wrote);
    }

}


