extern crate flate2;

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;

fn main() {
    //check input args
    if args().len() != 4 {
        eprintln!("Usage: <compress|decompress> <source> <target>");
        return;
    }
    //unwap for errorhandling
    let operation = args().nth(1).unwrap();
    let source_path = args().nth(2).unwrap();
    let target_path = args().nth(3).unwrap();
    //start timer
    let start = Instant::now();

    if operation == "compress" {
        //compression
        let mut input = BufReader::new(File::open(&source_path).unwrap());
        let output = File::create(&target_path).unwrap();
        let mut encoder = GzEncoder::new(output, Compression::default());

        copy(&mut input, &mut encoder).unwrap();
        let output = encoder.finish().unwrap();

        println!(
            "Source len: {:?}",
            input.get_ref().metadata().unwrap().len()
        );
        println!("Compressed len: {:?}", output.metadata().unwrap().len());

    } else if operation == "decompress" {
        // Decompression
        let input = File::open(&source_path).unwrap();
        let mut decoder = GzDecoder::new(BufReader::new(input));
        let mut output = BufWriter::new(File::create(&target_path).unwrap());

        copy(&mut decoder, &mut output).unwrap();

        println!("Decompressed file created at: {}", target_path);

    } else {
        eprintln!("Invalid operation: use 'compress' or 'decompress'");
        return;
    }

    println!("Elapsed: {:?}", start.elapsed());
}
