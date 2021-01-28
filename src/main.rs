/*
* Name:         Gabe Tiemann
* Assignment:   Homework 1
* Due Date:     Jan 25, 2020
* Class:        CS 494 - Safe & Eff. Programming in Rust
* Professor:    Jakob Eriksson
*/

use std::io::*;
// use std::fs::File;

fn main() -> Result<()> {

    // track which flags are given
    let mut byte_count = false;
    let mut word_count = false;
    let mut line_count = false;
    // grab arguments
    let args = BufReader::new(stdin());
    // store input file
    let mut filename: Vec<String> = Vec::new();

    for x in args.lines(){

        println!("{:?}", x);
        let input = x?;

        for y in input.split_ascii_whitespace(){
            // check that there is a flag present
            if y.contains("-"){
                println!("found flag!");
                // check for the type of flag
                if y.contains("c"){
                    byte_count = true;
                }
                if y.contains("w"){
                    word_count = true;
                }
                if y.contains("l"){
                    line_count = true;
                }
            }
            // otherwise if there is no flag
            else if y.contains(".txt"){
                filename.push(y.to_string());
            }
        }
    }
    // let contents = std::fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");

    // println!("With text:\n\n{}", contents);
    // println!("bytes: {}", contents.len());
    // println!("chars: {}", contents.chars().count());

    // let mut newLines: String = contents.lines().collect();
    // // TODO: maybe use lines().collect() ?
    // // https://doc.rust-lang.org/std/iter/trait.Iterator.html
    // let mut lineSub = newLines.next();
    // let mut count = 0;
    // let done = false;
    // for x in 0..10 {
    //     count += 1;
    //     println!("count: {}", count);
    //     println!("next: {:?}", lineSub);
    // }
    Ok(())
}


// /******** Helper Functions ********/


// /* Fn: print_bytes()
//     Desc: print info for all flags */
// fn print_bytes(){

// }
    

// /* Fn: print_words()
//     Desc: print info for all flags */
// fn print_words(){

// }


// /* Fn: print_lines()
//     Desc: print info for all flags */
// fn print_lines(){

// }


// /* Fn: print_all()
//     Desc: print info for all flags */
// fn print_all(){

// }