use regex::Regex;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("urlunrap regex1 regex2");
        println!("Match a line of stdin against regex1, then url decode it.");
        println!("Then pass the result to regex2 to extract and display to stdout");
        println!("regex2 must contain the named capture url: (?P<url>.*)");
        std::process::exit(0);
    };
    let pattern1 = &args[1];
    let pattern2 = &args[2];

    println!("{} - {}", &pattern1, &pattern2);
    
    let reg1 = match Regex::new(pattern1) {
        Ok(r) => r,
        Err(err) => {
            println!("urlunwrap: Error in pattern 1: {} - {}", pattern1, err);
            std::process::exit(1);
        }
    };

    let reg2 = match Regex::new(pattern2) {
        Ok(r) => r,
        Err(err) => {
            println!("urlunwrap: Error in pattern 2: {} - {}", pattern2, err);
            std::process::exit(1);
        }
    };

    println!("Using {} {}", &reg1, &reg2);
    println!("Send URL to stdin to unwrap");

    scan_stdin(&reg1, &reg2, io::stdin().lock());
    println!("Finished.");
}

fn scan_stdin<R>(reg1: &Regex, reg2: &Regex, reader: R) where R: BufRead
{
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(r) => r,
            Err(err) => {
                println!("urlunwrap: Problem reading from stdin - {}", err);
                break;
            }
        };
        if reg1.is_match(&line) {
            decode_string(&line, reg2);
        }
        println!("Waiting.");
    }
}

fn decode_string(line :&str, reg2: &Regex)
{
    let result = urldecode::decode(line);

    let caps = reg2.captures(&result).unwrap();
    let text1 = &caps["url"];
    println!("Result: {}", &text1);
}

fn hex_to_char(s: &str) -> char {
    char::from((u8::from_str_radix(s, 16)).unwrap())
}
