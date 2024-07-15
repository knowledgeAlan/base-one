mod base64_test;
use std::{ error::Error, process, str::from_utf8_mut};

 
use csv::Reader;
use base64_test::*;

struct Row {
    title:String,
    age:i128,
}
fn read_csv() -> Result<(),Box<dyn Error>> {
   
//    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut reader = Reader::from_path("test.csv")?;

   for result in reader.records() {
    let record: csv::StringRecord = result?;
    println!("{:?}", record);
    println!("{}",&record[0]);
    println!("{}",&record[1]);

   }
    Ok(())
}

fn main() {
    println!("Hello, world!");

    // if let Err(err) = read_csv() {
    //     print!("error running reader csv :{}",err);
    //     process::exit(1);
    // }

    encodeBase64String();
    decodeBase64String();
}
