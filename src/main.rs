/*
* Name:         Gabe Tiemann
* Assignment:   Homework 1
* Due Date:     Jan 25, 2020
* Class:        CS 494 - Safe & Eff. Programming in Rust
* Professor:    Jakob Eriksson
*/

// use std::io::*;

/*
    Testing for:
    - no parameters
        - read from stdin
    - flags
        * -w    words
        * -c    bytes
                - String.len() counts in bytes https://doc.rust-lang.org/std/string/struct.String.html#method.len 
        * -l    lines
                TRY: counting \n characters
        - passed with and without parameters
    - one or multiple files passed in
*/

fn main() {

    let args: Vec<String> = std::env::args().collect();

    // pseudo code
    // if ( &args[1] is NOT present )
    //     standard input
    // else 
    //     let filename = &args[1];

    let filename = &args[1];
    println!("In file {}", filename);

    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n\n{}", contents);
    println!("bytes: {}", contents.len());
    println!("chars: {}", contents.chars().count());

    let mut newLines = contents.lines();
    // let iterbitter = contents.iter();
    let mut count = 0;
    let done = false;
    for x in newLines.next() {
        // if newLines == NULL{
        //     done = true;
        // }
        count += 1;
        println!("count: {}", count);
        println!("next: {:?}", newLines.next());
    }
}
