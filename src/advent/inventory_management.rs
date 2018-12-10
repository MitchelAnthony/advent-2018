pub struct Inventory {
    boxes: Vec<String>
}

impl Inventory {
    pub fn new(boxes: Vec<String>) -> Inventory {
        Inventory {
            boxes,
        }
    }

    pub fn checksum(&self) -> isize {
        let mut double = 0;
        let mut triple = 0;

        for box_id in &self.boxes {
            double += Inventory::check_duplicate(&box_id, 2);
            triple += Inventory::check_duplicate(&box_id, 3);
        }

        double * triple
    }

    pub fn common_letters(&self) -> String {
        println!("{:#?}", self.boxes);        
        
        String::from("")
    }

    fn check_duplicate(string: &str, count: isize) -> isize {
        use std::collections::HashMap;

        let mut letters: HashMap<char, isize> = HashMap::new();

        for letter in string.chars() {
            let count = letters.entry(letter).or_insert(0);
            *count += 1;
        }

        for value in letters.values() {
            if count == *value {
                return 1;
            }
        }

        0
    }

    fn find_single_letter_diff(string_one: &str, string_two: &str) -> isize {
        let mut index = -1;
        let mut matches = 0;
        
        if string_one.len() != string_two.len() {
            return index;
        }

        index
    }
}

pub fn load_inventory(filename: &str) -> Vec<String> {
    use std::fs;
    
    let mut boxes: Vec<String> = Vec::new();
    let contents = fs::read_to_string(filename)
        .expect("Error reading file!");
    
    for box_id in contents.lines() {
        boxes.push(String::from(box_id));
    }

    boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut boxes = load_inventory("./src/resources/test/inventory_management_1.txt");
        let mut inventory = Inventory::new(boxes);
        assert_eq!(inventory.checksum(), 12);

        boxes = load_inventory("./src/resources/test/inventory_management_2.txt");
        inventory = Inventory::new(boxes);
        assert_eq!(inventory.common_letters(), "fgij");

        boxes = load_inventory("./src/resources/inventory_management.txt");
        inventory = Inventory::new(boxes);
        assert_eq!(inventory.checksum(), 6225);
    }
}
