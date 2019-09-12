use std::env;

const DOC_PAGES_PER_SHEET: usize = 4;
const DOC_PAGES_PER_SIGNATURE: usize = 16;
const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let args = &all_args[1..];
    if args.len() < 2 {
        println!("I need at least two page numbers! Received: {:?}", args);
        return;
    }
    let first_arg = &args[0];
    let second_arg = &args[1];
    let first_number: u32 = first_arg.parse().expect(&format!("First argument not a valid number! {}", first_arg));
    let second_number: u32 = second_arg.parse().expect(&format!("Second argument not a valid number! {}", second_arg));
    if first_number == 0 {
        println!("There is no page zero! Received: {:?}", args);
        return;
    }
    if second_number < first_number {
        println!("The second number must be greater than or equal to the first! Received: {:?}", args);
        return;
    }
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

