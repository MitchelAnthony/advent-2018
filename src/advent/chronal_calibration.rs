struct Device {
    frequency: isize,    
}

impl Device {
    fn new() -> Device {
        Device { frequency: 0 }
    }

    fn update_frequency(&mut self, change: isize) -> isize {
        self.frequency += change;

        self.frequency
    }
}

pub fn calibrate(filename: &str) -> isize {
    let mut tardis_device = Device::new();
    let frequency_changes = parse_input(filename);

    for change in frequency_changes.into_iter() {
        tardis_device.update_frequency(change);
    }

    tardis_device.frequency
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
    fn test_chronal_calibration() {
        assert_eq!(calibrate("./src/resources/test/chronal_calibration_1.txt"), 76);
        assert_eq!(calibrate("./src/resources/test/chronal_calibration_2.txt"), 3);
        assert_eq!(calibrate("./src/resources/chronal_calibration.txt"), 477);
    }
}
