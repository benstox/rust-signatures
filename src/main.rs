use std::env;
use std::process;

use rust_signatures::DocumentInfo;


fn main() {
    let all_args: Vec<String> = env::args().collect();
    let (first_number, second_number) = rust_signatures::parse_args(all_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let document_info = DocumentInfo::new(first_number, second_number);
    document_info.display();
}

// Number of document pages to print: 60
// Number of sheets to print: 15
// Number of 4-sheet signatures to bind: 4
// #####################################
// Signature A. First page: 1, last page: 16
// Signature B. First page: 17, last page: 32
// Signature C. First page: 33, last page: 48
// Signature D. First page: 49, last page: 60
// #####################################
