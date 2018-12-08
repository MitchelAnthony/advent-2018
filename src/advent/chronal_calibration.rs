pub struct Device {
    frequency: isize,
    natural_frequency: Option<isize>,
    history: Vec<isize>,
}

impl Device {
    pub fn new() -> Device {
        Device { 
            frequency: 0,
            natural_frequency: None,
            history: vec![0],
        }
    }

    pub fn update_frequencies(&mut self, changes: &Vec<isize>) -> isize {
        for change in changes {
            self.update_frequency(*change);
        }

        self.frequency
    }

    pub fn update_frequency(&mut self, change: isize) -> isize {
        self.frequency += change;
        self.history.push(self.frequency);

        self.frequency
    }

    pub fn update_natural_frequency(&mut self) -> Option<isize> {
        use std::collections::HashMap;

        let mut frequencies: HashMap<isize, isize> = HashMap::new();

        for frequency in &self.history {
            if frequencies.get(frequency).is_some() {
                self.natural_frequency = Some(*frequency);
                return self.natural_frequency;
            }

            frequencies.insert(*frequency, 1);
        }

        None
    }
}

fn parse_input(filename: &str) -> Vec<isize> {
    use std::fs;

    let contents = fs::read_to_string(filename)
        .expect("Error reading file!");
    
    let mut numbers = Vec::new();
    for line in contents.lines() {
        numbers.push(line.parse::<isize>().unwrap());
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut tardis_device = Device::new();
        let mut frequencies = parse_input("./src/resources/test/chronal_calibration_1.txt");
        let mut frequency = tardis_device.update_frequencies(&frequencies);
        let mut natural_frequency = tardis_device.update_natural_frequency();
        
        assert_eq!(frequency, 76);
        assert_eq!(natural_frequency, None);


        tardis_device = Device::new();
        frequencies = parse_input("./src/resources/test/chronal_calibration_2.txt");
        frequency = tardis_device.update_frequencies(&frequencies);
        natural_frequency = tardis_device.update_natural_frequency();

        assert_eq!(frequency, -5);
        assert_eq!(natural_frequency, Some(-8));


        tardis_device = Device::new();
        frequencies = parse_input("./src/resources/chronal_calibration.txt");
        frequency = tardis_device.update_frequencies(&frequencies);
        loop {
            natural_frequency = tardis_device.update_natural_frequency();
            if natural_frequency.is_some() {
                break;
            }

            tardis_device.update_frequencies(&frequencies);
        }
        
        assert_eq!(frequency, 477);
        assert_eq!(natural_frequency, Some(390));
    }
}
