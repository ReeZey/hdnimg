mod reader;
mod utils;
mod writer;

use std::{path::PathBuf, str::FromStr, time::SystemTime};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct HDNIMG {
    #[arg(long, default_value="1")]
    bit_count: usize,

    #[arg(long)]
    input_file: String,
    
    #[arg(long)]
    output_file: String,

    #[arg(long)]
    data: String
}

fn main() {
    println!("brrr");
    let hdnimg = HDNIMG::parse();
    let total_time = SystemTime::now();
    
    if hdnimg.bit_count == 0 || hdnimg.bit_count > 8 {
        panic!("invalid bit count, valid range: [1 - 8] not {:?}", hdnimg.bit_count)
    }
    
    let input_file = PathBuf::from_str(&hdnimg.input_file).unwrap();
    let output_file = PathBuf::from_str(&hdnimg.output_file).unwrap();
    
    let start = SystemTime::now();
    writer::encode(hdnimg.data.bytes().collect(), &input_file, &output_file, &hdnimg.bit_count).expect("could not encode image, reason");
    println!("Encode took {} ms", SystemTime::now().duration_since(start).unwrap().as_millis());
    
    println!();
    println!(" ---- ");
    println!();
    
    let start = SystemTime::now();
    let buffer = reader::decode(&output_file, &hdnimg.bit_count).expect("could not decode image, reason");
    println!("Decode took {} ms", SystemTime::now().duration_since(start).unwrap().as_millis());
    
    let str = String::from_utf8_lossy(&buffer);
    
    assert_eq!(hdnimg.data, str);
    
    println!("Success! your image now contains the following text {:?} encoded in bit_count {} bits!", str, hdnimg.bit_count);
    println!("Total time {} ms", SystemTime::now().duration_since(total_time).unwrap().as_millis());
}
