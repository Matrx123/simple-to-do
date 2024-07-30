use std::collections::HashMap;

pub struct Todo {
    list: HashMap<i32, String>,
    curr_index: i32,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            list: HashMap::new(),
            curr_index: 0,
        }
    }

    pub fn todo(self) -> HashMap<i32, String> {
        self.list
    }

    pub fn add(&mut self, items: String) -> &str {
        self.list.insert(self.curr_index + 1, items);
        self.curr_index += 1;
        "todo item added!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_todo_items_to_a_list() {
        let mut list = Todo::new();
        list.add(String::from("apple"));
        list.add(String::from("banana"));
        list.add(String::from("guvava"));
        list.add(String::from("grapes"));
        let result = list.curr_index;
        assert_eq!(result, 4);
    }
}
