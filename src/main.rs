#![allow(unstable)]

use game::{Game, Guessed};

use std::io;
use std::fmt::{Show, Error, Formatter};

mod game;

fn main() {
    println!("Guess the word game");
    let intended_word = get_word();
    let mut game = Game::new(intended_word);
    game_loop(&mut game);
    println!("'{}' - You won!", game.intended_word());
}

fn game_loop(game : &mut Game) {
    loop {
        print!(" {:?}, {} letters left, ", game.current_progress(), game.letters_left());
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        let input_trimed = input.as_slice().trim();
        game.guess(input_trimed);
        if game.letters_left() == 0 {
            break;
        }
    }
}

fn get_word() -> String {
    "abcdefghijklmnopqrstuvwxyz".to_string()
}

impl Show for Vec<Guessed> {
    fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
        let mut result = String::with_capacity(self.len() * 2);
        for guess in self.iter() {
            match *guess {
                Guessed::Yes(ch) => result.push(ch),
                Guessed::No => result.push('_'),
            }
            result.push(' ');
        }
        write!(f, "{}", result.as_slice())
    }
}
