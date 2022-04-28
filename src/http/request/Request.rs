use std::collections::HashMap;
use std::error::Error;

pub struct HttpRequest{
    method:String,
    path:String,
    text:String,
    //headerString:String,
    //bodyString:String,
}

impl HttpRequest{
    pub fn new(text:&str)->Self{
        let split_by_linebreak = text.lines();
        let lbVec:Vec<&str> = split_by_linebreak.collect();
        let spVec:Vec<&str> = lbVec[0].split(" ").collect();
        let method:&str = spVec[0];
        let path:&str = spVec[1];

        println!("method:{}",spVec[0]);
        println!("path:{}",spVec[1]);
        println!("lbIterator : {}",lbVec[0]);
        //split_by_linebreak.for_each(|str|println!("{}112",str));

        return HttpRequest{
            text:text.to_string(),
            path:path.to_string(),
            method:method.to_string()
        };

    }
 
}

