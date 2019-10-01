use std::env;

const DOC_PAGES_PER_SHEET: u32 = 4;
const DOC_PAGES_PER_SIGNATURE: u32 = 16;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Debug)]
struct Signature {
   first_page: u32,
   last_page: u32,
   signature_key: String,
}

#[derive(Debug)]
struct DocumentInfo {
    num_pages: u32,
    num_sheets: u32,
    num_signatures: u32,
    signatures: Vec<Signature>,
}

impl DocumentInfo {
    fn new(first_number: u32, second_number: u32) -> DocumentInfo {
        // Calculate the number of pages, sheets and signatures in the document.
        let num_pages = (second_number - first_number + 1) as u32;
        let num_sheets = (num_pages as f32 / DOC_PAGES_PER_SHEET as f32).ceil() as u32;
        let num_signatures = (num_pages as f32 / DOC_PAGES_PER_SIGNATURE as f32).ceil() as u32;
        let signatures = get_signatures(first_number, num_pages, num_signatures);
        DocumentInfo {
            num_pages,
            num_sheets,
            num_signatures,
            signatures,
        }
    }

    fn display(&self) {
        println!("Number of document pages to print: {}", self.num_pages);
        println!("Number of sheets to print: {}", self.num_sheets);
        println!("Number of 4-sheet signatures to bind: {}", self.num_signatures);
        println!("#####################################");
        for signature in &self.signatures {
            println!(
                "Signature {}. First page: {}, last_page: {}",
                signature.signature_key,
                signature.first_page,
                signature.last_page,
            )
        }
        println!("#####################################");
    }
}

fn parse_args(all_args: Vec<String>) -> (u32, u32) {
    // Convert the command line arguments to the numbers we need and
    // make sure they are sensible.
    println!("{:?}", all_args);
    let args = &all_args[1..]; // 0th element is name of the binary
    if args.len() < 2 {
        panic!("Need at least two arguments to run! Got: {:?}", args)
    }
    let first_arg = &args[0];
    let second_arg = &args[1];
    let first_number: u32 = first_arg.parse().expect(&format!("First argument not a valid number! {}", first_arg));
    let second_number: u32 = second_arg.parse().expect(&format!("Second argument not a valid number! {}", second_arg));
    if first_number == 0 {
        panic!("There is no page zero! Received {} as a first arg.", first_number);
    }
    if second_number < first_number {
        panic!("The second number must be greater than or equal to the first! Received {} as a second arg.", second_number);
    }
    (first_number, second_number)
}

fn get_signatures(first_page_of_document: u32, num_pages: u32, num_signatures: u32) -> Vec<Signature> {
    // get the starting and ending pages of each signature in the document
    let last_page_of_document = first_page_of_document + num_pages - 1;
    let mut signatures: Vec<Signature> = Vec::new();
    for i in 0..num_signatures {
        let last_page_of_signature = ((i + 1) * DOC_PAGES_PER_SIGNATURE) + first_page_of_document - 1;
        let signature = Signature {
            first_page: (DOC_PAGES_PER_SIGNATURE * i) + first_page_of_document,
            last_page: if last_page_of_signature < last_page_of_document {
                last_page_of_signature
            } else {
                last_page_of_document
            },
            signature_key: get_signature_key(i),
        };
        signatures.push(signature);
    }
    signatures
}

fn get_signature_key(signature_i: u32) -> String {
    // get the letter code that identifies each signature
    let mut key = String::new();
    let mut i = signature_i as usize;
    loop {
        let remainder = i % ALPHABET.len(); 
        key.push_str(&ALPHABET[remainder..remainder + 1]);
        i = i / ALPHABET.len();
        if i == 0 {
            break;
        }
        i = i - 1;
    }
    key.chars().rev().collect() // needs to be reversed since we're appending to the right
}

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let (first_number, second_number) = parse_args(all_args);
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

#[test]
fn test_get_signature_key() {
    assert_eq!(get_signature_key(0), "A");
    assert_eq!(get_signature_key(1), "B");
    assert_eq!(get_signature_key(2), "C");
    assert_eq!(get_signature_key(25), "Z");
    assert_eq!(get_signature_key(26), "AA");
    assert_eq!(get_signature_key(27), "AB");
    assert_eq!(get_signature_key(51), "AZ");
    assert_eq!(get_signature_key(52), "BA");
    assert_eq!(get_signature_key(78), "CA");
    assert_eq!(get_signature_key(701), "ZZ");
    assert_eq!(get_signature_key(702), "AAA");
    assert_eq!(get_signature_key(703), "AAB");
}

#[test]
fn test_get_signatures() {
    // simple example
    let first_number = 1;
    let num_pages = 16;
    let num_signatures = 1;
    let signatures = get_signatures(first_number, num_pages, num_signatures); 
    assert!(signatures.len() == 1);
    assert!(signatures[0].first_page == 1);
    assert!(signatures[0].last_page == 16);

    // not using all the pages of a signature
    let first_number = 1;
    let num_pages = 9;
    let num_signatures = 1;
    let signatures = get_signatures(first_number, num_pages, num_signatures);
    assert!(signatures.len() == 1);
    assert!(signatures[0].first_page == 1);
    assert!(signatures[0].last_page == 9);

    // more than one signature
    let first_number = 1;
    let num_pages = 19;
    let num_signatures = 2;
    let signatures = get_signatures(first_number, num_pages, num_signatures);
    assert!(signatures.len() == 2);
    assert!(signatures[0].first_page == 1);
    assert!(signatures[0].last_page == 16);
    assert!(signatures[1].first_page == 17);
    assert!(signatures[1].last_page == 19);

    // not starting from the first page
    let first_number = 5;
    let num_pages = 19;
    let num_signatures = 2;
    let signatures = get_signatures(first_number, num_pages, num_signatures);
    assert!(signatures.len() == 2);
    println!("{:?}", signatures);
    assert!(signatures[0].first_page == 5);
    assert!(signatures[0].last_page == 20);
    assert!(signatures[1].first_page == 21);
    assert!(signatures[1].last_page == 23);
}

#[test]
fn test_document_info_new() {
    // smallest possible
    let document_info = DocumentInfo::new(1, 1);
    assert_eq!(document_info.num_pages, 1);
    assert_eq!(document_info.num_sheets, 1);
    assert_eq!(document_info.num_signatures, 1);

    // full sheet
    let document_info = DocumentInfo::new(1, 4);
    assert_eq!(document_info.num_pages, 4);
    assert_eq!(document_info.num_sheets, 1);
    assert_eq!(document_info.num_signatures, 1);

    // not starting at 1
    let document_info = DocumentInfo::new(7, 8);
    assert_eq!(document_info.num_pages, 2);
    assert_eq!(document_info.num_sheets, 1);
    assert_eq!(document_info.num_signatures, 1);

    // larger one
    let document_info = DocumentInfo::new(1, 60);
    assert_eq!(document_info.num_pages, 60);
    assert_eq!(document_info.num_sheets, 15);
    assert_eq!(document_info.num_signatures, 4);
    
    // larger one not starting at 1
    let document_info = DocumentInfo::new(12, 30);
    assert_eq!(document_info.num_pages, 19);
    assert_eq!(document_info.num_sheets, 5);
    assert_eq!(document_info.num_signatures, 2);
}

#[test]
fn test_parse_args() {
    let (first_number, second_number) = parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "1".to_string(),
        "60".to_string(),
    ]);
    assert_eq!(first_number, 1);
    assert_eq!(second_number, 60);

    // can be the same number twice
    let (first_number, second_number) = parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "33".to_string(),
        "33".to_string(),
    ]);
    assert_eq!(first_number, 33);
    assert_eq!(second_number, 33);

    // doesn't matter if it gets extra args
    let (first_number, second_number) = parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "5".to_string(),
        "185".to_string(),
        "asdfasdfad".to_string(),
    ]);
    assert_eq!(first_number, 5);
    assert_eq!(second_number, 185);
}

#[test]
#[should_panic(expected = "Need at least two arguments to run!")]
fn test_parse_args_insufficient_args() {
    parse_args(vec![
        "5".to_string(),
    ]);
}

#[test]
#[should_panic(expected = "First argument not a valid number!")]
fn test_parse_args_first_arg_not_number() {
    parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "asdfasd".to_string(),
        "60".to_string(),
    ]);
}

#[test]
#[should_panic(expected = "Second argument not a valid number!")]
fn test_parse_args_second_arg_not_number() {
    parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "345".to_string(),
        "asdfa60".to_string(),
    ]);
}

#[test]
#[should_panic(expected = "There is no page zero!")]
fn test_parse_args_first_arg_zero() {
    parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "0".to_string(),
        "60".to_string(),
    ]);
}

#[test]
#[should_panic(expected = "The second number must be greater than")]
fn test_parse_args_second_arg_smaller() {
    parse_args(vec![
        "target/debug/rust-signatures".to_string(),
        "33".to_string(),
        "32".to_string(),
    ]);
}

