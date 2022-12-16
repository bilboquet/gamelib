use crate::GameStatus;

pub(crate) struct Board {
    pub stick_count: usize,
    stick_init: usize,
}

impl Board {
    pub(crate) fn new(stick_count: usize) -> Self {
        Self {
            stick_count,
            stick_init: stick_count,
        }
    }

    pub(crate) fn apply_move(&mut self, player1_move: crate::guesser::Move) -> GameStatus {
        match player1_move {
            crate::guesser::Move::One => self.stick_count -= 1,
            crate::guesser::Move::Two => self.stick_count -= 2,
        }

        if self.stick_count == 0 {
            return GameStatus::Win;
        }
        GameStatus::OnGoing
    }

    pub(crate) fn display(&self) {
        for n in 0..self.stick_count {
            print!("| ");
        }
        println!();
    }

    pub fn reset(&mut self) {
        self.stick_count = self.stick_init;
    }
}
