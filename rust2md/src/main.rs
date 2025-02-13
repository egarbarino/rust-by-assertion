use std::io::{self, BufRead};
use std::env;
use std::process;

struct State {
    started_parsing: bool,
    start_of_comment_at: usize,
    parsing_code : bool,
}

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("rust2md is a utility that converts a Rust file to markdown following these rules:");
        println!();
        println!("1. Code is ignored until the first comment appears");
        println!("2. Comments using // are interpreted as containing markdown text");
        println!("3. Comments using /* */ are not parsed");
        println!("4. Code following comments will be included using the markdown code fence notation");
        println!("5. Code should start in the last column as the last comment");
        println!("");
        println!("Usage: rust2md < input.rs > output.md");
        process::exit(0);
    }

    let mut state = State{
        started_parsing : false,
        start_of_comment_at : 0,
        parsing_code : false,
    };

    //let file = &args[1];

    //let file = File::open(file)?; // Open the file
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = io::BufReader::new(handle); // Create a buffered reader
    for line in reader.lines() {
        let line = line?; // Handle potential errors
        let line_trimmed = line.trim_start();
        // 
        // We found comments // which we assume to contain markdown text
        //
        if line_trimmed.starts_with("//") {
            //
            // If we were parsing code before, we need to close the
            // code snippet
            //
            if state.started_parsing == true && state.parsing_code == true {
                println!("```");
                println!();
            }
            state.started_parsing = true;
            state.parsing_code = false;
            state.start_of_comment_at = line.find('/').unwrap();

            if line_trimmed.len() < 3 {
                /* Empty comment */
                println!();
            } else {
                println!("{}", &line[state.start_of_comment_at+3..]);
            }
        } else if state.started_parsing == true {
            //
            // If we haven't found a comment, then we assume we
            // are embedding a code snippet
            //
            if state.parsing_code == false {
                    println!();
                    println!("``` rust");
                    state.parsing_code = true;
            }
            if line.len() >= state.start_of_comment_at {
                println!("{}", &line[state.start_of_comment_at..]);
            } else if line.len() == 0 { /* Empty line */
                println!();
            }
        }
    }
    // If the last thing we did was parse a code snippet,
    // we need to close it
    if state.started_parsing == true && state.parsing_code == true {
        println!("```");
        println!();
    }

    Ok(())
}
