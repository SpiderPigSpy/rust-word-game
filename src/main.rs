#![feature(io)]
#![feature(rand)]
#![allow(deprecated)]

use game::{Game};

use std::old_io;

mod game;

fn main() {
    println!("Guess the word game");
    let mut game = Game::new(get_words());
    game_loop(&mut game);
    println!("\n'{}' - You won!", game.intended_word());
}

fn game_loop(game : &mut Game) {
    loop {
        print!(" {}, {} letters left, ", game.current_progress(), game.letters_left());
        let input = old_io::stdin().read_line().ok().expect("Failed to read line");
        let input_trimed = &input[].trim();
        game.guess(input_trimed);
        if game.letters_left() == 0 {
            break;
        }
    }
}

fn get_words() -> Vec<String> {
    let words = vec![
        "rust",
        "is",
        "the",
        "best",
        "programming",
        "language",
    ];
    words.iter().map(|x| x.to_string()).collect()
}
