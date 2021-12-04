use std::collections::BTreeSet;

pub enum HasWon {
    True,
    False,
}

pub struct Board {
    /// board[row][column]
    pub board: [[Option<u8>; 5]; 5],
    pub numbers_remaining: BTreeSet<u8>,
}

impl Board {
    /// Are we done?
    fn has_won(&self) -> HasWon {
        for row in self.board {
            if row.iter().all(|x| x.is_none()) {
                return HasWon::True;
            }
        }
        for col_num in 0usize..5 {
            let mut so_far_so_good = true;
            for row_num in 0usize..5 {
                if let Some(_) = self.board[row_num][col_num] {
                    so_far_so_good = false;
                    break;
                }
            }
            if so_far_so_good {
                return HasWon::True;
            }
        }
        HasWon::False
    }

    /// A new number has been called - update the grid!
    pub fn call(&mut self, number: u8) -> HasWon {
        if self.numbers_remaining.remove(&number) {
            for values in self.board.iter_mut() {
                for option_value in values.iter_mut() {
                    if let Some(value) = option_value {
                        if value == &number {
                            *option_value = None;
                            return self.has_won();
                        }
                    }
                }
            }
            panic!("Logic error")
        }
        HasWon::False
    }

    pub fn construct_final_answer(&self, last_number_called: u8) -> u32 {
        self.numbers_remaining
            .iter()
            .map(|x| x.clone() as u32)
            .sum::<u32>()
            * (last_number_called as u32)
    }

    pub fn new_from_str(input: &str) -> Self {
        let lines = input.split('\n');
        let mut board = [[Some(0u8); 5]; 5];
        let mut numbers_remaining = BTreeSet::new();

        for (i, row) in lines.enumerate() {
            for (j, number) in row.split_ascii_whitespace().enumerate() {
                let n: u8 = number.parse().unwrap();
                numbers_remaining.insert(n);
                board[i][j] = Some(n);
            }
        }

        Board {
            board,
            numbers_remaining,
        }
    }
}
