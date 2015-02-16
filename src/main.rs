#![feature(io)]
#![feature(rand)]
#![feature(path)]
#![allow(deprecated)]

use game::{Game};

use std::old_io;
use std::old_io::BufferedReader;
use std::old_io::File;

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
    let path = Path::new("words.txt");
    let mut file = BufferedReader::new(File::open(&path));
    file.lines().map(|x| x.unwrap().trim().to_string()).collect()
}
