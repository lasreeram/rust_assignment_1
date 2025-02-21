// Can you build a clap based command line program that uses the marco_polo function?
//
// The program should:
// - Accept a name as a command line argument
// - Call the marco_polo function with the name
// - Print the result to the console
// - If no name is provided, the program should print an error message and exit
// - If the name is "marco", the program should print "polo"

use clap::{App, Arg};

fn main() {
    let matches = App::new("Marco Polo")
        .version("1.0")
        .author("Sreeram")
        .about("A Marco Polo game in Rust")
        .arg(
            Arg::with_name("name")
                .help("Your name")
                .required(true)
                .index(1),
        )
        .get_matches(); // Parse the command line arguments

    let name = matches.value_of("name").unwrap(); // Get the value of the name argument

    let response = hello_marco::marco_polo(name); // Call the marco_polo function

    println!("{}", response); // Print the response


}