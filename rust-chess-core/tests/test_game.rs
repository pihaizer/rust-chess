use rust_chess_core::board::PieceColor;
use rust_chess_core::game::{Game};

#[test]
fn test_pgn_games() {
    let files = std::fs::read_dir("./tests/pgn_games").unwrap();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        if let Some(extension) = path.extension() && extension == "pgn" {
            let content = std::fs::read_to_string(path).unwrap();
            test_pgn_game(&content);
        }
    }
}

fn test_pgn_game(pgn_game: &str) {
    let mut checkmate_or_stalemate = false;
    let mut is_game = false;
    let mut game: Game = Game::new();
    let mut i = 0;

    'lines: for line in pgn_game.lines() {
        if !is_game {
            if line.starts_with("[Termination") {
                if line.contains("won by checkmate") || line.contains("drawn by stalemate") {
                    checkmate_or_stalemate = true;
                }
                continue;
            } else if line.starts_with("1.") {
                is_game = true;
                game = Game::new();
                i = 0;
            } else { continue }
        }

        for mv in line.split_whitespace() {
            if mv == "1/2-1/2" || mv == "1-0" || mv == "0-1" {
                if checkmate_or_stalemate {
                    assert_game_result_str(&game, mv);
                }
                checkmate_or_stalemate = false;
                is_game = false;
                continue 'lines;
            }

            if i % 3 == 0 {
                i += 1;
                continue;
            }
            i += 1;

            let is_mate = mv.ends_with("#");
            let is_check = is_mate || mv.ends_with("+");

            let mv = match game.parse_short_notation(mv) {
                Ok(mv) => mv,
                Err(err) => {
                    game.board().print(true);
                    panic!("Failed to parse move {}: {}", mv, err);
                }
            };

            match game.make_move(&mv) {
                Ok(_) => {}
                Err(err) => {
                    game.board().print(true);
                    panic!("Failed to make move {}: {}", mv, err);
                }
            }

            assert_eq!(is_check, game.is_check());
            if is_mate {
                assert!(game.result().is_some())
            }
        }
    }
}

fn assert_game_result_str(game: &Game, game_result_str: &str) {
    let game_result = game.result();
    if game_result_str == "1/2-1/2" {
        assert!(game_result.is_some());
        assert_eq!(game_result.as_ref().unwrap().winner, None);
    } else if game_result_str == "1-0" {
        assert!(game_result.is_some());
        assert_eq!(
            game_result.as_ref().unwrap().winner,
            Some(PieceColor::White)
        );
    } else if game_result_str == "0-1" {
        assert!(game_result.is_some());
        assert_eq!(
            game_result.as_ref().unwrap().winner,
            Some(PieceColor::Black)
        );
    } else {
        panic!("Invalid game result in PGN: {}", game_result_str);
    }
}