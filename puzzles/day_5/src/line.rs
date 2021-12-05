pub struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    pub fn is_horiz_or_vert(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    /// Move left to right, going up or down
    fn get_diagonal_points(&self) -> Vec<(u32, u32)> {
        let (start, end) = if self.start.0 <= self.end.0 {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        };


        if start.1 <= end.1 {  // Going up
            (start.0..=end.0).enumerate().map(|(i, x)| (x, start.1 + i as u32)).collect()
        } else {
            (start.0..=end.0).enumerate().map(|(i, x)| (x, start.1 - i as u32)).collect()
        }
    }

    pub fn get_points(&self) -> Vec<(u32, u32)> {
        if self.start.0 == self.end.0 {
            let min = self.start.1.min(self.end.1);
            let max = self.start.1.max(self.end.1);
            (min..=max).map(|i| (self.start.0, i)).collect()
        } else if self.start.1 == self.end.1 {
            let min = self.start.0.min(self.end.0);
            let max = self.start.0.max(self.end.0);
            (min..=max).map(|i| (i, self.start.1)).collect()
        } else {
            self.get_diagonal_points()
        }
    }

    pub fn parse(s: &str) -> Self {
        let make_tuple = |x: &str| {
            let mut splits = x.split(",");
            (
                splits.next().unwrap().parse::<u32>().unwrap(),
                splits.next().unwrap().parse::<u32>().unwrap(),
            )
        };
        let mut splits = s.split(" -> ").map(make_tuple);
        let start = splits.next().unwrap();
        let end = splits.next().unwrap();
        Self { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "0,9 -> 5,9";
        let line = Line::parse(s);
        assert_eq!(line.start, (0, 9));
        assert_eq!(line.end, (5, 9));
    }

    #[test]
    fn test_get_points() {
        let line = Line {
            start: (0, 9),
            end: (5, 9),
        };
        let points = line.get_points();
        let expected = vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)];
        assert_eq!(points, expected);

        let backwards_line = Line {
            start: (4, 3),
            end: (4, 1),
        };
        let points = backwards_line.get_points();
        let expected = vec![(4, 1), (4, 2), (4, 3)];
        assert_eq!(points, expected);

        let smol_line = Line {
            start: (1, 1),
            end: (1, 1),
        };
        let points = smol_line.get_points();
        let expected = vec![(1, 1)];
        assert_eq!(points, expected);
    }
}
