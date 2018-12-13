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

    pub fn overlap_area(first: &Claim, second: &Claim) -> usize {
        let mut area: usize = 0;

        // There are three cases to consider
        // One: the claims do not overlap
        if second.bottom_left.y <= first.top_left.y // second is above first
            || second.top_left.y >= first.bottom_left.y // second is below first
            || second.top_right.x <= first.top_left.x // second is to the left of first
            || second.top_left.x >= first.top_right.x { // second is to the right of first
            return 0;
        }

        // Two: one is completely inside the other
        if second.is_inside(&first) {
            return second.area();
        } else if first.is_inside(&second) {
            return first.area();
        }

        // Three: the claims partially overlap
        if second.top_left.x >= first.top_left.x {
            (first.top_right.x - second.top_left.x) * if second.top_left.y >= first.top_left.y {
                (first.bottom_right.y - second.top_left.y)
            } else {
                (second.bottom_left.y - first.top_right.y)
            }    
        } else if first.top_left.x >= second.top_left.x {
            (second.top_right.x - first.top_left.x) * if first.top_left.y <= second.top_left.y {
                (first.bottom_left.y - second.top_right.y)
            } else {
                (second.bottom_right.y - first.top_left.y)
            }
        } else {
            0 // This should never be reached
        }
    }

    pub fn area(&self) -> usize {
        (self.top_right.x - self.top_left.x) * (self.bottom_right.y - self.top_right.y)
    }

    pub fn is_inside(&self, second: &Claim) -> bool {
        self.top_left.x >= second.top_left.x
            && self.top_left.y >= second.top_left.y
            && self.bottom_right.x <= second.bottom_right.x
            && self.bottom_right.y <= second.bottom_right.y
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

    #[test]
    fn test_is_inside() {
        let claim_one = Claim {
            id: 1,
            top_left: Edge { x: 0, y: 0 },
            top_right: Edge { x: 10, y: 0 },
            bottom_left: Edge { x: 0, y: 10 },
            bottom_right: Edge { x: 10, y: 10 },
        };

        let claim_two = Claim {
            id: 2,
            top_left: Edge { x: 2, y: 0 },
            top_right: Edge { x: 8, y: 0 },
            bottom_left: Edge { x: 2, y: 5 },
            bottom_right: Edge { x: 8, y: 5 },
        };

        let claim_three = Claim {
            id: 3,
            top_left: Edge { x: 5, y: 6 },
            top_right: Edge { x: 15, y: 6 },
            bottom_left: Edge { x: 5, y: 11 },
            bottom_right: Edge { x: 15, y: 11 },
        };

        assert_eq!(claim_one.is_inside(&claim_two), false);
        assert_eq!(claim_two.is_inside(&claim_one), true);
        assert_eq!(claim_three.is_inside(&claim_one), false);
    }

    #[test]
    fn test_overlap_area_not_overlapping() {
        let claim_one = Claim {
            id: 1,
            top_left: Edge { x: 0, y: 0 },
            top_right: Edge { x: 10, y: 0 },
            bottom_left: Edge { x: 0, y: 10 },
            bottom_right: Edge { x: 10, y: 10 },
        };

        let claim_two = Claim {
            id: 2,
            top_left: Edge { x: 20, y: 0 },
            top_right: Edge { x: 30, y: 0 },
            bottom_left: Edge { x: 20, y: 5 },
            bottom_right: Edge { x: 30, y: 5 },
        };

        assert_eq!(Claim::overlap_area(&claim_one, &claim_two), 0);
    }

    #[test]
    fn test_overlap_area_completely_overlapping() {
        let claim_one = Claim {
            id: 1,
            top_left: Edge { x: 0, y: 0 },
            top_right: Edge { x: 10, y: 0 },
            bottom_left: Edge { x: 0, y: 10 },
            bottom_right: Edge { x: 10, y: 10 },
        };

        let claim_two = Claim {
            id: 2,
            top_left: Edge { x: 2, y: 0 },
            top_right: Edge { x: 8, y: 0 },
            bottom_left: Edge { x: 2, y: 5 },
            bottom_right: Edge { x: 8, y: 5 },
        };

        assert_eq!(Claim::overlap_area(&claim_one, &claim_two), 30);
    }

    #[test]
    fn test_overlap_area_partially_overlapping() {
        let claim_one = Claim {
            id: 1,
            top_left: Edge { x: 0, y: 0 },
            top_right: Edge { x: 10, y: 0 },
            bottom_left: Edge { x: 0, y: 10 },
            bottom_right: Edge { x: 10, y: 10 },
        };

        let claim_two = Claim {
            id: 2,
            top_left: Edge { x: 5, y: 5 },
            top_right: Edge { x: 15, y: 5 },
            bottom_left: Edge { x: 5, y: 15 },
            bottom_right: Edge { x: 15, y: 15 },
        };

        assert_eq!(Claim::overlap_area(&claim_one, &claim_two), 25);
        assert_eq!(Claim::overlap_area(&claim_two, &claim_one), 25);

        let claim_three = Claim {
            id: 3,
            top_left: Edge { x: 0, y: 10 },
            top_right: Edge { x: 10, y: 10 },
            bottom_left: Edge { x: 0, y: 20 },
            bottom_right: Edge { x: 10, y: 20 },
        };

        assert_eq!(Claim::overlap_area(&claim_two, &claim_three), 25);
        assert_eq!(Claim::overlap_area(&claim_three, &claim_two), 25);
    }
}
