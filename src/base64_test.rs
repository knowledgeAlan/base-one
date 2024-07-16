use core::fmt;
use std::string;
use base64::prelude::*;

pub fn encodeBase64String(input:&str)->String {
     
     println!("{}",input );
     let encode = BASE64_STANDARD.encode(input);
     return  encode;
    
}

pub fn decodeBase64String(input:&str)-> String {
    let res =BASE64_STANDARD.decode(input).unwrap();
    
      return String::from_utf8(res).unwrap(); 
    
}