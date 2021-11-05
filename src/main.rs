use memmap::MmapOptions;
use std::{env};
use std::fs::File;
use std::io;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("--------------------------- start --------------------------------");
    let args: Vec<String> = env::args().collect();
    println!("Hello: {:?}", &args);
    if args.len() < 2 {
        println!("please specify csv file.");
        return Ok(());
    }

    let filename = args.get(1).unwrap();
    let file = File::open(filename)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    println!("size={}", mmap.len());
    Ok(())
}
