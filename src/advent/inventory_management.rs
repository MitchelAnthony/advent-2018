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
        let mut id_one = String::from("");
        let mut id_two = String::from("");
        let mut index = -1;

        for (key, box_id_one) in self.boxes.iter().enumerate() {
            for box_id_two in self.boxes.iter().skip(key + 1) {
                let i = Inventory::find_single_letter_diff(&box_id_one, &box_id_two);
                if i != -1 {
                    index = i;
                    id_one = box_id_one.clone();
                    id_two = box_id_two.clone();
                    break;
                }
            }
        }

        if id_one.len() == 0 {
            return String::from("");
        }

        let mut letters = String::from("");
        for (key, letter) in id_one.chars().enumerate() {
            if (key as isize) == index {
                continue;
            }
            letters.push(letter);
        }

        letters
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
        let mut differences = 0;
        
        if string_one.len() != string_two.len() {
            return -1;
        }

        let mut chars_one = string_one.chars().enumerate().peekable();
        let mut chars_two = string_two.chars();
        loop {
            if chars_one.peek().is_none() {
                break;
            }
            let (key, char_one) = chars_one.next().unwrap();
            let char_two = chars_two.next().unwrap();

            if char_one != char_two {
                index = key as isize;
                differences += 1;
            }
        }

        if differences == 1 {
            index
        } else {
            -1
        }
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
        assert_eq!(inventory.common_letters(), "revtaubfniyhsgxdoajwkqilp");
    }

    #[test]
    fn test_find_differences() {
        let mut s1 = String::from("test");
        let mut s2 = String::from("tezt");
        assert_eq!(Inventory::find_single_letter_diff(&s1, &s2), 2);

        s1 = String::from("applebananapear");
        s2 = String::from("applebananopear");
        assert_eq!(Inventory::find_single_letter_diff(&s1, &s2), 10);

        s1 = String::from("apple");
        s2 = String::from("apple");
        assert_eq!(Inventory::find_single_letter_diff(&s1, &s2), -1);

        s1 = String::from("apple");
        s2 = String::from("apples");
        assert_eq!(Inventory::find_single_letter_diff(&s1, &s2), -1);

        s1 = String::from("apple");
        s2 = String::from("azzle");
        assert_eq!(Inventory::find_single_letter_diff(&s1, &s2), -1);
    }
}
