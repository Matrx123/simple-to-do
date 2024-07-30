use std::{env, io};

use todo_app::Todo;

//Simple ToDo cli in RUST
fn main() {
    println!("CLI for TODO App 🦀");
    println!("--------------------");
    let args: Vec<String> = env::args().collect();
    println!("args >>> {:?}", args);
    println!("enter a todo command or '--help' for todo commands!");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong!!");

    let command: Vec<&str> = input.trim().split(' ').collect();

    let mut list = Todo::new();
    list.add(String::from("vipin joshi"));
    let result = list.todo();
    println!("{:?}", result);
}
