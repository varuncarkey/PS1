//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitorCount: int = 0;

fn main() {

    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        
        unsafe{
        visitorCount += 1;
        }
        
                
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);

            let path: ~[&str] = request_str.split(' ').collect();
            let html:  ~[&str] =path[1].split('.').collect();
            

            if path[1] != "/" && html.len()>1 && html[1]=="html"{
                
                let path_file = Path::new(path[1].slice_from(1));
                let file_content = File::open(&path_file);

                match (file_content) {
                    Some(mut content) => {
                        let content_bytes: ~[u8] = content.read_to_end();
                        stream.write(content_bytes);
                        println!("Connection terminates.");
                    } ,
                    None => fail!("Error opening message file: {:s}", path[1])
                
                }
            }

           else if path[1] != "/"
            {
                let response: ~str = 
                ~"HTTP/1.1 403 Forbidden\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>403 Error</title>
                 <link rel='stylesheet' href='//netdna.bootstrapcdn.com/bootstrap/3.0.3/css/bootstrap.min.css'>
                 <link rel='stylesheet' href='//netdna.bootstrapcdn.com/bootstrap/3.0.3/css/bootstrap-theme.min.css'>
                 </head>
                 <body>
                 <div class='jumbotron'>
                    <h1>403 Error</h1>
                    <p>Forbidden. File was not HTML type</p>
                  </div>
                </body></html>\r\n";
            stream.write(response.as_bytes());
            println!("Connection terminates.");

            }else{
            
             let response: ~str = 
                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 <p>Visitor Count : "+ unsafe{visitorCount.to_str()}  +"</p>
                 </body></html>\r\n";
            stream.write(response.as_bytes());
            println!("Connection terminates.");
            }
            
            println(format!("Received request :\n{:s}", request_str));
            
            
        }
    }
}
