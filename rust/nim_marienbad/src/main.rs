mod board;
use board::Board;

mod guesser;
use guesser::Guesser;

mod score;

#[derive(Debug, PartialEq)]
enum GameStatus {
    OnGoing,
    Loose,
    Win,
    Draw,
}

fn main() {
    let board_size = 30usize;

    let mut board = Board::new(board_size);
    let mut score = score::Scorer::new(2);

    let mut player1 = Guesser::new(board_size);

    loop {
        let mut player2 = Guesser::new(board_size);

        let status = board.apply_move(player1.guess(&board));

        if status == GameStatus::Win {
            println!("Player one wins!");
            score.winner(1);
            board.reset();
            player1.reinforce(status);
            player1.display_hints();
            continue;
        }

        let status = board.apply_move(player2.guess(&board));

        if status == GameStatus::Win {
            println!("Player two wins!");
            score.winner(2);
            board.reset();
            player1.reinforce(GameStatus::Loose);
            continue;
        }
    }
}
