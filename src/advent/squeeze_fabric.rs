pub struct Edge {
    x: usize,
    y: usize,
}

pub struct Claim {
    id: usize,
    top_left: Edge,
    top_right: Edge,
    bottom_left: Edge,
    bottom_right: Edge,
}

impl Claim {
    pub fn from(claim_description: &str, claim_regex: &regex::Regex) -> Claim {
        let mut id: usize = 0;
        let mut top_left: Edge = Edge { x: 0, y: 0 };
        let mut length: usize = 0;
        let mut width: usize = 0;
        
        for captures in claim_regex.captures_iter(claim_description) {
            id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            top_left.x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            top_left.y = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            length = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
            width = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();
        }

        Claim {
            id,
            top_left: Edge { x: top_left.x, y: top_left.y },
            top_right: Edge { x: top_left.x + length, y: top_left.y },
            bottom_left: Edge { x: top_left.x, y: top_left.y + width },
            bottom_right: Edge { x: top_left.x + length, y: top_left.y + width },
        }
    }

    pub fn overlap_units(first: &Claim, second: &Claim) -> usize {
        let mut units: usize = 0;

        

        units
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        use regex::Regex;

        let claim_regex = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();
        let mut claim_description = String::from("#1 @ 1,3: 4x4");
        let mut claim = Claim::from(&claim_description, &claim_regex);
        
        assert_eq!(claim.top_left.x, 1);
        assert_eq!(claim.top_left.y, 3);
        assert_eq!(claim.bottom_right.x, 5);
        assert_eq!(claim.bottom_right.y, 7);

        claim_description = String::from("#908 @ 207,344: 21x28");
        claim = Claim::from(&claim_description, &claim_regex);
        
        assert_eq!(claim.top_left.x, 207);
        assert_eq!(claim.top_left.y, 344);
        assert_eq!(claim.top_right.x, 228);
        assert_eq!(claim.top_right.y, 344);
        assert_eq!(claim.bottom_left.x, 207);
        assert_eq!(claim.bottom_left.y, 372);
        assert_eq!(claim.bottom_right.x, 228);
        assert_eq!(claim.bottom_right.y, 372);
    }
}
