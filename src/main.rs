use std::env;

const DOC_PAGES_PER_SHEET: usize = 4;
const DOC_PAGES_PER_SIGNATURE: usize = 16;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn parse_args(all_args: Vec<String>) -> (u32, u32) {
    // convert the command line arguments to the numbers we need and
    // make sure they are sensible
    println!("{:?}", all_args);
    let args = &all_args[1..]; // 0th element is name of the binary
    if args.len() < 2 {
        panic!("Need at least two arguments to run! Got: {:?}", args)
    }
    let first_arg = &args[0];
    let second_arg = &args[1];
    // for some reason it doesn't like the ? operator on these parses:
    // I get type mismatch "expected struct `std::num::ParseIntError`, found &str"
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

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let (first_number, second_number) = parse_args(all_args);
    let num_pages = (second_number - first_number + 1) as usize;
    let num_sheets = (num_pages as f32 / DOC_PAGES_PER_SHEET as f32).ceil() as usize;
    let num_signatures = (num_pages as f32 / DOC_PAGES_PER_SIGNATURE as f32).ceil() as usize;
    println!("Number of document pages to print: {}", num_pages);
    println!("Number of sheets to print: {}", num_sheets);
    println!("Number of 4-sheet signatures to bind: {}", num_signatures);
    println!("#####################################");
    for (i, letter) in ALPHABET.chars().enumerate() {
        if i as usize == num_signatures {
            break;
        }
        let first_page = (i * DOC_PAGES_PER_SIGNATURE) + 1;
        let last_page = if (i + 1) * DOC_PAGES_PER_SIGNATURE < num_pages {
            (i + 1) * DOC_PAGES_PER_SIGNATURE
        } else {
            num_pages  
        };
        println!("Signature {}. First page: {}, last page: {}", letter, first_page, last_page);
    }
    println!("#####################################");
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

