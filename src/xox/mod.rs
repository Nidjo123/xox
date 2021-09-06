#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    Circle,
    Cross,
    Empty,
}

const SIZE: usize = 3;

struct TicTacToe {
    state: [[Symbol; SIZE]; SIZE],
    current_player: Symbol,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            state: [[Symbol::Empty; SIZE]; SIZE],
            current_player: Symbol::Cross,
        }
    }

    pub fn play_move(&mut self, row: usize, col: usize) -> Result<Option<Symbol>, String> {
        if row >= SIZE {
            Err("invalid row".to_owned())
        } else if col >= SIZE {
            Err("invalid column".to_owned())
        } else {
            match self.state[row][col] {
                Symbol::Empty => {
                    self.state[row][col] = self.current_player;
                    self.current_player = self.next_player();
                    self.print_state();
                    Ok(self.winner())
                }
                _ => Err("not empty".to_owned())
            }
        }
    }

    fn next_player(&mut self) -> Symbol {
        match self.current_player {
            Symbol::Circle => Symbol::Cross,
            Symbol::Cross => Symbol::Circle,
            Symbol::Empty => panic!("cannot determine next player"),
        }
    }

    fn check_straight(&self) -> Option<Symbol> {
        // rows
        for i in 0..SIZE {
            let symbol = self.state[i][0];
            if symbol != Symbol::Empty && self.state[i][1..SIZE].iter().all(|&x| x == symbol) {
                return Some(symbol);
            }
        }
        // cols
        for i in 0..SIZE {
            let symbol = self.state[0][i];
            if symbol != Symbol::Empty && self.state.iter()
                                                    .skip(1)
                                                    .map(|&x| x[i])
                                                    .all(|x| x == symbol) {
                return Some(symbol);
            }
        }
        None
    }

    fn check_diagonal(&self) -> Option<Symbol> {
        let mut main_diagonal = vec![];
        let mut other_diagonal = vec![];
        for i in 0..SIZE {
            main_diagonal.push(self.state[i][i]);
            other_diagonal.push(self.state[i][SIZE - i - 1]);
        }
        if main_diagonal.iter().all(|&x| x == main_diagonal[0] && x != Symbol::Empty) {
            return Some(main_diagonal[0]);
        } else if other_diagonal.iter().all(|&x| x == other_diagonal[0] && x != Symbol::Empty) {
            return Some(other_diagonal[0]);
        }
        None
    }

    fn winner(&self) -> Option<Symbol> {
        self.check_straight()
            .or(self.check_diagonal())
            .or(match self.state.iter().flatten().any(|&x| x == Symbol::Empty) {
                true => None,
                false => Some(Symbol::Empty), // no winner and no empty places
            })
    }

    fn print_state(&self) {
        for row in &self.state {
            println!("{:?}", row);
        }
        println!("Next move: {:?}", self.current_player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_run() {
        let mut xox = TicTacToe::new();
        assert_eq!(xox.play_move(1, 1), Ok(None));
        assert_eq!(xox.play_move(1, 1), Err("not empty".to_owned()));
        assert_eq!(xox.play_move(0, 0), Ok(None));
        assert_eq!(xox.play_move(0, 1), Ok(None));
        assert_eq!(xox.play_move(1, 0), Ok(None));
        assert_eq!(xox.play_move(2, 2), Ok(None));
        assert_eq!(xox.play_move(2, 0), Ok(Some(Symbol::Circle)));
    }

    #[test]
    fn no_winner() {
        let mut xox = TicTacToe::new();
        assert_eq!(xox.play_move(2, 2), Ok(None));
        assert_eq!(xox.play_move(0, 0), Ok(None));
        assert_eq!(xox.play_move(0, 1), Ok(None));
        assert_eq!(xox.play_move(1, 2), Ok(None));
        assert_eq!(xox.play_move(2, 0), Ok(None));
        assert_eq!(xox.play_move(0, 0), Err("not empty".to_owned()));
        assert_eq!(xox.play_move(2, 1), Ok(None));
        assert_eq!(xox.play_move(0, 2), Ok(None));
        assert_eq!(xox.play_move(1, 1), Ok(None));
        assert_eq!(xox.play_move(1, 0), Ok(Some(Symbol::Empty)));
    }
}
