struct Device {
    frequency: isize,
    history: Vec<isize>,
}

impl Device {
    fn new() -> Device {
        Device { 
            frequency: 0,
            history: vec![0]
        }
    }

    fn update_frequency(&mut self, change: isize) -> isize {
        self.frequency += change;
        self.history.push(self.frequency);

        self.frequency
    }

    fn duplicate_frequency(&self) -> Result<isize, bool> {
        use std::collections::HashMap;
        let mut frequency_map: HashMap<isize, isize> = HashMap::new();

        for frequency in &self.history {
            match frequency_map.get(&frequency) {
                Some(i) => return Ok(*i),
                _ => (),
            };

            frequency_map.insert(*frequency, *frequency);
        }

        Err(false)
    }
}

pub fn calibrate(filename: &str) -> isize {
    let tardis_device = prepare_device(filename);

    tardis_device.frequency
}

pub fn duplicate_frequency(filename: &str) -> Result<isize, bool> {
    let tardis_device = prepare_device(filename);

    tardis_device.duplicate_frequency()
}

fn prepare_device(filename: &str) -> Device {
    let mut tardis_device = Device::new();
    let frequency_changes = parse_input(filename);

    for change in frequency_changes {
        tardis_device.update_frequency(change);
    }

    tardis_device
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
    fn test_calibrate() {
        assert_eq!(calibrate("./src/resources/test/chronal_calibration_1.txt"), 76);
        assert_eq!(calibrate("./src/resources/test/chronal_calibration_2.txt"), -5);
        assert_eq!(calibrate("./src/resources/chronal_calibration.txt"), 477);
    }

    #[test]
    fn test_duplicate_frequency() {
        assert_eq!(duplicate_frequency("./src/resources/test/chronal_calibration_1.txt"), Err(false));
        assert_eq!(duplicate_frequency("./src/resources/test/chronal_calibration_2.txt"), Ok(-8));
        assert_eq!(duplicate_frequency("./src/resources/chronal_calibration.txt"), Ok(0));
    }
}
