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
        let input_string: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.next() {
            Some(keyword) => match keyword {
                "add" => {
                    let result: &str = list.add(input_string[1].to_string());
                    println!("{result}");
                }
                "fetch_all" => {
                    println!("---------------------");
                    for (key, val) in list.get_all().iter() {
                        println!("{:?} : {:?}", key, val);
                    }
                    println!("---------------------");
                }
                "fetch_one" => {
                    println!("---------------------");
                    let result = list.get_by_id(input_string[1].parse::<i32>().unwrap());
                    if result.is_ok() {
                        println!("Item : {:?}", result.unwrap());
                    } else {
                        println!("Item doesn't exist for this ID, please enter a valid ID");
                    }
                    println!("---------------------");
                }
                "delete_one" => {
                    println!("---------------------");
                    let result = list.delete(input_string[1].parse::<i32>().unwrap());
                    if result.is_ok() {
                        println!("Item Deleted!!");
                    } else {
                        println!("Item doesn't exists for this ID, please enter a valid ID");
                    }
                    println!("---------------------");
                }
                "update_one" => {
                    println!("---------------------");
                    let result =
                        list.update_one(input_string[1].parse::<i32>().unwrap(), input_string[2]);
                    if result.is_ok() {
                        println!("{:?}", result.unwrap());
                    } else {
                        println!("{:?}", result.unwrap_err());
                    }
                    println!("----------------------");
                }
                _ => println!(
                    "Please enter a valid TODO command, enter '--help' to see available commands!!"
                ),
            },
            None => println!("Empty operation!!, please try again!!"),
        }

        if input_string.len() == 1 && input_string[0] == "--help" {
            println!("TODO commands:\n1- add 'task name'\n2- delete 'todo item id'\n3- update 'item id' 'updated text'");
            println!("---------------------");
        }
    }
}
