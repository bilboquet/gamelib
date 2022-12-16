use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use crate::board;
use crate::GameStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    One,
    Two,
}

#[derive(Debug, Clone)]
struct Hint {
    take_one: usize,
    take_two: usize,
}

impl Hint {
    pub fn is_empty(&self) -> bool {
        self.take_one == 0 && self.take_two == 0
    }

    pub fn reinforce(&mut self, mv: Move) {
        match mv {
            Move::One => self.take_one += 2,
            Move::Two => self.take_two += 2,
        }
    }

    pub fn punish(&mut self, mv: Move) {
        match mv {
            Move::One => self.take_one -= 1,
            Move::Two => self.take_two -= 1,
        }

        if self.is_empty() {
            *self = DEFAULT_HINT.clone();
        }
    }
}

static DEFAULT_HINT: Hint = Hint {
    take_one: 2usize,
    take_two: 2usize,
};

#[derive(Debug)]
pub(crate) struct Guesser {
    hints: Vec<Hint>,
    rng: ChaCha20Rng,
    played_moves: Vec<(usize, Move)>,
}

impl Guesser {
    pub fn display_hints(&self) {
        for hint in self.hints.iter() {
            let total = hint.take_one + hint.take_two;
            if hint.take_one >= hint.take_two {
                print!("<P(1):{}> ", 100 * hint.take_one  / total );
            } else {
                print!("<P(2):{}> ", 100 * hint.take_two  / total );
            }
        }
        println!();
    }

    pub fn guess(&mut self, board: &board::Board) -> Move {
        let position = board.stick_count - 1;
        let guess = &mut self.hints[position];

        assert!(!guess.is_empty());

        let my_move = if board.stick_count == 1 {
            Move::One
        } else {
            let possible_move_count = guess.take_one + guess.take_two;
            let rand = self.rng.gen_range(1..=possible_move_count);
            if rand <= guess.take_one && guess.take_one > 0 {
                Move::One
            } else if guess.take_two > 0 {
                Move::Two
            } else {
                panic!("no guess can be made")
            }
        };

        self.played_moves.push((position, my_move));

        my_move
    }

    pub fn new(size: usize) -> Self {
        Self {
            hints: vec![DEFAULT_HINT.clone(); size],
            rng: ChaCha20Rng::from_entropy(),
            played_moves: Vec::new(),
        }
    }

    pub fn reinforce(&mut self, status: GameStatus) {
        match status {
            GameStatus::Loose => {
                for (pos, mv) in self.played_moves.iter() {
                    self.hints[*pos].punish(*mv);
                }
                self.played_moves.clear();
            }
            GameStatus::Win => {
                for (pos, mv) in self.played_moves.iter() {
                    self.hints[*pos].reinforce(*mv);
                }
                self.played_moves.clear();
            }
            GameStatus::OnGoing => todo!(),
            GameStatus::Draw => todo!(),
        }
    }
}
