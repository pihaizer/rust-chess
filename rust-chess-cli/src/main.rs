use rust_chess_core::game::Game;
use rust_chess_core::r#move::Move;
use std::io;
use std::io::Write;

fn main() {
    let mut game = Game::new();
    let mut input = String::new();
    loop {
        // read command from the console
        game.board().print();
        print!("Your move: ");
        io::stdout().flush().unwrap();
        
        input.clear();
        let input_result = io::stdin().read_line(&mut input);
        if input_result.is_err() {
            println!("Error reading input: {}", input_result.err().unwrap());
            return;
        }
        
        let command = input.trim();
        let mv = Move::from_long_notation(command);
        let move_result = game.make_move(&mv);
        if move_result.is_err() {
            println!("Error: {}", move_result.err().unwrap());
        }
        if let Some(game_result) = game.result() {
            println!("Game over!");
            if let Some(winner_color) = game_result.winner {
                println!("Winner: {:?}", format!("{}", winner_color));
            } else {
                println!("It's a draw!");
            }
            break;
        }
    }

    // game.board().print();
    // game.make_move(Move::new(1, 1, 1, 3)).unwrap();
    // game.board().print();
    // game.make_move(Move::new(1, 6, 1, 4)).unwrap();
    // game.board().print();
    // game.make_move(Move::new(2, 1, 2, 3)).unwrap();
    // game.board().print();
    // game.make_move(Move::new(4, 6, 1, 3)).unwrap();
    // game.board().print();
    // game.make_move(Move::from_long_notation("c4b5")).unwrap();
    // game.board().print();
}
