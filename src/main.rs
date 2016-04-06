#[macro_use]
extern crate log;
extern crate env_logger;

extern crate coio;
extern crate http_muncher;

use coio::Scheduler;
use coio::net::tcp::TcpListener;
use coio::net::tcp::Shutdown;

// Include the 2 main public interfaces of the crate
use http_muncher::{Parser, ParserHandler};

// Now let's define a new listener for parser events:
struct CoioHttpHandler;
impl ParserHandler for CoioHttpHandler {

    // Now we can define our callbacks here.
    //
    // Let's try to handle headers: the following callback function will be
    // called when parser founds a header in the HTTP stream.
    fn on_url(&mut self, url: &[u8]) -> bool {
        //println!("{:?}: ", ::std::str::from_utf8(url).unwrap());

        true
    }

    fn on_header_field(&mut self, header: &[u8]) -> bool {
        // Print the received header key
        // println!("{:?}: ", ::std::str::from_utf8(header).unwrap());

        true
    }

    // And let's print the header values in a similar vein:
    fn on_header_value(&mut self, value: &[u8]) -> bool {
        // println!("\t {:?}", ::std::str::from_utf8(value).unwrap());
        true
    }
}

fn main() {
    env_logger::init().unwrap();

    let mut resp = String::new();
    resp.push_str("HTTP/1.1 200 OK\r\n");
    //resp.push_str("Connection: close\r\n");
    resp.push_str("Content-Length: 10\r\n");
    resp.push_str("\r\n");
    resp.push_str("abcdefghij");


    Scheduler::new()
        .with_workers(1)
        .run(move || {
            let server = TcpListener::bind("127.0.0.1:3000").unwrap();

            info!("Listening on {:?}", server.local_addr().unwrap());

            for stream in server.incoming() {
                use std::io::{Read, Write};


                let (mut stream, addr) = stream.unwrap();
                info!("Accept connection: {:?}", addr);

                let resptext = resp.clone();
                Scheduler::spawn(move || {
                    let mut buf = [0; 1024 * 8];

                    // Now we can create a parser instance with our callbacks handler:
                    let callbacks_handler = CoioHttpHandler;
                    let mut parser = Parser::request(callbacks_handler);

                    loop {
                        debug!("Trying to Read...");
                        match stream.read(&mut buf) {
                            Ok(0) => {
                                debug!("EOF received, going to close");
                                //stream.shutdown(Shutdown::Both);
                                break;
                            }
                            Ok(len) => {
                                info!("Read {} bytes, echo back!", len);

                                parser.parse(&buf[0..len]);

                                if let Err(err) = stream.write_all(resptext.as_bytes()) {
                                    println!("write error.");
                                    break;
                                }

                            }
                            Err(err) => {
                                //stream.shutdown(Shutdown::Both);
                                // panic!("Error occurs: {:?}", err);
                                break;
                            }
                        }
                    }

                    info!("{:?} closed", addr);
                });
            }
        })
        .unwrap();
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
