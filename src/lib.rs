use unicode_segmentation::UnicodeSegmentation;
pub struct Life {
    live_cell_character: String,
    dead_cell_character: String,
    line_delimiter_character: String,
    width: usize,
    height: usize,
}

impl Life {
    pub fn new(w: usize, h: usize) -> Self {
        return Life {
            live_cell_character: String::from("█"),
            dead_cell_character: String::from(" "),
            line_delimiter_character: String::from("\n"),
            width: w,
            height: h,
        };
    }

    pub fn wrap_coordinate(i: isize, max: isize) -> isize {
        (i % max + max) % max
    }

    pub fn position_to_index(&self, x: isize, y: isize) -> usize {
        let x = Life::wrap_coordinate(x, self.width as isize);
        let y = Life::wrap_coordinate(y, self.height as isize);
        let index = ((self.width as isize) * y + x) as usize;
        let total = self.width * self.height;
        self::Life::wrap_coordinate(index as isize, total as isize) as usize
    }

    pub fn tick(&self, input: &String) -> String {
        let mut output: String = String::new();
        let graphemes: &Vec<String> = &input
            .graphemes(false)
            .filter(|s| *s != "\n")
            .map(|s| String::from(s))
            .collect();

        for (i, x, y) in (0..self.width * self.height).map(|i| (i, i % self.width, i / self.width))
        {
            if i > 0 && i % self.width == 0 {
                output.push_str("\n");
            }
            let next = self.get_next_state(graphemes, y as isize, x as isize);
            output.push_str(&next);
        }
        output
    }

    fn get_next_state(&self, graphemes: &Vec<String>, y: isize, x: isize) -> String {
        let char = &graphemes[(y * self.width as isize + x) as usize];
        if *char == self.line_delimiter_character {
            return self.line_delimiter_character.clone();
        }
        let neighbors = self.count_neighbors(graphemes, y, x);
        return if *char == self.live_cell_character && (neighbors == 2 || neighbors == 3) {
            // we have a living cell
            self.live_cell_character.to_string()
        } else if *char == self.dead_cell_character && neighbors == 3 {
            self.live_cell_character.to_string()
        } else {
            self.dead_cell_character.to_string()
        };
    }

    pub fn count_neighbors(&self, graphemes: &Vec<String>, y: isize, x: isize) -> i32 {
        let positions = &[
            self.position_to_index(x - 1, y - 1),
            self.position_to_index(x, y - 1),
            self.position_to_index(x + 1, y - 1),
            self.position_to_index(x - 1, y),
            self.position_to_index(x + 1, y),
            self.position_to_index(x - 1, y + 1),
            self.position_to_index(x, y + 1),
            self.position_to_index(x + 1, y + 1),
        ];
        let mut neighbors = 0;
        for index in positions {
            neighbors = if graphemes[*index] == self.live_cell_character {
                neighbors + 1
            } else {
                neighbors
            };
        }
        neighbors
    }
}

#[cfg(test)]
mod test {
    use unicode_segmentation::UnicodeSegmentation;

    use crate::Life;

    #[test]
    fn positions_are_normalized() {
        assert_eq!(Life::wrap_coordinate(-12, 3), 0);
        assert_eq!(Life::wrap_coordinate(6, 3), 0);
        assert_eq!(Life::wrap_coordinate(-11, 3), 1);
    }

    #[test]
    fn position_to_index_wraps_correctly() {
        let life = Life::new(3, 3);
        let positions_with_indices = [
            ((-1_isize, -1_isize), 8_usize),
            ((3_isize, 0_isize), 0_usize),
        ];
        for ((x, y), i) in positions_with_indices.iter() {
            let index = life.position_to_index(*x, *y);
            assert_eq!(index, *i);
        }
    }

    #[test]
    fn tick_kills_from_overpopulation() {
        let life = Life::new(3, 3);
        let seed = "███\n███\n███";
        let output = life.tick(&seed.to_string());
        assert_eq!(output, "   \n   \n   ");
    }

    #[test]
    fn tick_produces_from_reproduction() {
        let life = Life::new(3, 3);
        let seed = "█  \n█  \n█  ";
        let output = life.tick(&seed.to_string());
        assert_eq!(output, "███\n███\n███");
    }

    #[test]
    fn count_neighbors() {
        let tests = [
            (String::from("█████████"), (0_isize, 0_isize), 8),
            (String::from("█  \n█  \n█  "), (0_isize, 0_isize), 2),
        ];
        let life = Life::new(3, 3);
        for (seed, (x, y), expectation) in tests.iter() {
            let seed = &seed.graphemes(true).map(|s| String::from(s)).collect();
            assert_eq!(life.count_neighbors(seed, *x, *y), *expectation);
        }
    }
    #[test]
    fn get_next_state() {
        let life = Life::new(3, 3);
        let tests = [("█  █  █  ", (0_isize, 0_isize), "█")];
        for (seed, (x, y), expected) in tests.iter() {
            assert_eq!(
                life.get_next_state(
                    &String::from(*seed)
                        .graphemes(true)
                        .map(|s| String::from(s))
                        .collect(),
                    *y,
                    *x
                ),
                String::from(*expected)
            )
        }
    }
}
