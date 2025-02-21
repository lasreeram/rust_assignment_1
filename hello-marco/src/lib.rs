/*
A Marco Polo game in Rust

If you gave the name 'marco' to a Rust program, it would respond with 'polo' when you ran it. This is a simple game that demonstrates how to write a command-line program in Rust.

else the program will ask 'What is your name?' and respond with 'Hello, <name>!'
*/

//Generate the code based on the above
pub fn marco_polo(name: &str) -> String {
    if name == "marco" {
        "polo".to_string()
    } else {
        format!("Hello, {}!", name)
    }
}
