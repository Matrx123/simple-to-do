use std::{env, io};

use todo_app::Todo;

//Simple ToDo cli in RUST
fn main() {
    println!("CLI for TODO App 🦀");
    println!("enter a todo command or '--help' for todo commands!");
    println!("--------------------");

    loop {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 && args[1] == "help" {
            println!("Todo commands:\n1- add 'task name'\n2- delete 'todo item id'\n3- update 'item id' 'updated text'");
            println!("--------------------");
        }

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!!");

        let command: Vec<&str> = input.trim().split(' ').collect();
        println!("{:?}", command);
    }
}
