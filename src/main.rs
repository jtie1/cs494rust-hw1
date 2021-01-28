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
    let mut i = 0;
    let mut total_words = 0;
    let mut total_lines = 0;
    let mut total_bytes = 0;
    // track which flags are given
    let mut byte_ = false;
    let mut word_ = false;
    let mut line_ = false;
    // grab arguments
    let args = BufReader::new(stdin());
    // store input file
    let mut filenames: Vec<String> = Vec::new();

    for x in args.lines(){

        let input = x?; // doing for y in x.split...() is invalid :(

        for y in input.split_ascii_whitespace(){
            // check that there is a flag present
            if y.contains("-"){
                // check for the type of flag
                if y.contains("c"){
                    byte_ = true;
                }
                if y.contains("w"){
                    word_ = true;
                }
                if y.contains("l"){
                    line_ = true;
                }
            }
            // otherwise if there is no flag
            else if y.contains("."){
                filenames.push(y.to_string());   // just stores the name of file
            }
        }

        // search each file from the input string

        for _e in &filenames{
            
            // count amount for this specific file
            let mut word_count = 0;
            let mut line_count = 0;
            // open the file
            let mut file = std::fs::File::open(&filenames[i])
                .expect("Unable to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            // get number of words
            for word in contents.split_ascii_whitespace(){
                word_count += 1;
            }
        
            // get number of lines
            for line in contents.chars(){
                if line == '\n'{
                    line_count += 1;
                }
            }

            // print results
            // if all flags are given or no flags are given
            if (byte_ && word_ && line_) || (!byte_ && !word_ && !line_){
                println!("  {}  {}  {}  {}", line_count, word_count, contents.len(), filenames[i]);
            }
            // -wl or lw
            else if !byte_ && word_ && line_{
                println!("  {}  {}  {}", word_count, line_count, filenames[i]);
            }
            // -cl or lc
            else if byte_ && !word_ && line_{
                println!("  {}  {}  {}", line_count, contents.len(), filenames[i]);
            }
            // -cw or wc
            else if byte_ && word_ && !line_{
                println!("  {}  {}  {}", word_count, contents.len(), filenames[i]);
            }
            // -c
            else if byte_ && !word_ && !line_{
                println!("  {}  {}", contents.len(), filenames[i]);
            }
            // -w
            else if !byte_ && word_ && !line_{
                println!("  {}  {}", word_count, filenames[i]);
            }
            // -l
            else if !byte_ && !word_ && line_{
                println!("  {}  {}", line_count, filenames[i]);
            }

            total_words += word_count;
            total_lines += line_count;
            total_bytes += contents.len();
            i += 1;
        }
        if i > 1{
            println!("  {}  {}  {}  total", total_lines, total_words, total_bytes);
        }
    }
    
    // let contents = std::fs::read_to_string(filenames)
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