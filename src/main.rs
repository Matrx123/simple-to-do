use std::io;

use todo_app::Todo;

//Simple ToDo cli in RUST
fn main() {
    println!("CLI for TODO App 🦀");
    println!("enter a todo command or '--help' for todo commands!");
    println!("--------------------");
    let mut list = Todo::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong!!");

        let mut tokens = input.trim().split_whitespace();
        match tokens.next() {
            Some(keyword) => match keyword {
                "add" => {
                    let input_string: Vec<&str> = input.trim().split_whitespace().collect();
                    let result: &str = list.add(input_string[1].to_string());
                    println!("{result}");
                }
                "get_all" => {
                    println!("ToDo List :: {:?}", list.get_all());
                }
                _ => println!(
                    "Please enter a valid TODO command, enter '--help' to see available commands!!"
                ),
            },
            None => println!("Empty operation!!, please try again!!"),
        }

        let command: Vec<&str> = input.trim().split(' ').collect();

        if command.len() == 1 && command[0] == "--help" {
            println!("TODO commands:\n1- add 'task name'\n2- delete 'todo item id'\n3- update 'item id' 'updated text'");
            println!("---------------------");
        }
    }
}
