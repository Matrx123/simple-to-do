use std::collections::HashMap;

pub struct Todo {
    list: HashMap<i32, String>,
    next_index: i32,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            list: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn get_all(&self) -> &HashMap<i32, String> {
        &self.list
    }

    pub fn get_by_id(&self, id: i32) -> Result<String, String> {
        match self.list.get(&id) {
            Some(&ref item) => Ok(item.to_string()),
            _ => Err(String::from("Doesn't exist!")),
        }
    }

    pub fn add(&mut self, items: String) -> &str {
        self.list.insert(self.next_index, items);
        self.next_index += 1;
        "todo item added!"
    }

    pub fn delete(&mut self, id: i32) -> Result<usize, &str> {
        if self.list.remove(&id).is_some() {
            Ok(self.list.len())
        } else {
            Err("Item not found!!")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    fn setup() -> Todo {
        let mut list = Todo::new();
        list.add(String::from("apple"));
        list.add(String::from("banana"));
        list.add(String::from("guava"));
        list.add(String::from("grapes"));
        list
    }

    #[test]
    fn add_todo_items_to_a_list() {
        let mut list = Todo::new();
        list.add(String::from("apple"));
        list.add(String::from("banana"));
        list.add(String::from("guava"));
        list.add(String::from("grapes"));

        let result = list.get_all().len();
        assert_eq!(result, 4);
    }

    #[test]
    fn fetch_all_todo_items() {
        let list = setup();
        let result = list.get_all().len();
        assert_eq!(result, 4);
    }

    #[test]
    fn fetch_item_by_id() {
        let list = setup();
        let result = list.get_by_id(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("banana"));
    }

    #[test]
    fn fetch_non_exist_item_by_id() {
        let list = setup();
        let result = list.get_by_id(20);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Doesn't exist!")
    }

    #[test]
    fn delete_an_item_from_list() {
        let mut list = setup();
        let result = list.delete(2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn delete_non_existing_item() {
        let mut list = setup();
        let result = list.delete(10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Item not found!!");
    }
}
