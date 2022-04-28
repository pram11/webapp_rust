use std::env;
use std::any;
use std::iter::Iterator;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod http;
use http::request::Request;

fn handle_connection(mut stream:TcpStream)->String{
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let requestString:String = String::from_utf8_lossy(&buffer[..]).to_string();
    println!("{}",&requestString);
    return requestString;
}
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut filename:&str;
    if args.len()==1{
        filename = "0.0.0.0:8000";
    }else{
        filename = &args[1];

    }
    
    println!("{}",filename);
    let mut splitted = filename.split(":");
    let split_count = splitted.clone().count();
    if (split_count!=2){
        println!("path not configured error");
        return;
    }
    let address = splitted.nth(0);
    match address{
        None=>{
            println!("failed!");
            return;
        },
        Some(value)=>{
            println!("address:{}",value);
            value;
        }
    };
    let address = address.unwrap();
    println!("println after address:{:?}",address);
    let port = splitted.nth(0);
    match port{
        None=>{
            println!("failed");
            return;
        }
        Some(val)=>{
            println!("port:{}",val);
            val;
        }
    }
    let port = port.unwrap();
    let port:i32 = port.parse().unwrap();
    println!("port:{}",port);
    let listener = TcpListener::bind(filename).unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection Established");
        let responseString:String = handle_connection(stream);

        println!("responseString:{}",responseString);
        let httpRequest = Request::HttpRequest::new(&responseString[..]);

    }

    // println!("Hello, world!");
    // let mut path:String = &args[1];
    // if path.contains(":"){
    //     let host:Vec<&str> = path.split(":");
    //     host.foreach(|item,iter|{
    //         println!("{}",item);
    //     });

}
