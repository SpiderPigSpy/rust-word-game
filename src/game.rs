use std::fmt::{Display, Error, Formatter};

pub struct Game {
    intended_word: String,
    guessed: Vec<GuessedRes>,
}

impl Game {
    pub fn new(intended_word: String) -> Game {
        let guessed = intended_word.chars().map(|ch| GuessedRes::new(ch, false)).collect();
        Game {
            intended_word: intended_word,
            guessed : guessed,
        }
    }

    pub fn intended_word<'r>(&'r self) -> &'r str {
        &self.intended_word[]
    }

    pub fn current_progress(&self) -> CurrentProgress {
        CurrentProgress { vec : self.guessed.iter().map(|x| x.to_enum()).collect() }
    }

    pub fn letters_left(&self) -> usize {
        self.guessed.iter().filter(|&x| !x.is_guessed).count()
    }

    pub fn guess(&mut self, guess : &str) {
        for ch in guess.chars() {
            self.guess_char(ch);
        }
    }

    fn guess_char(&mut self, ch : char) {
        for g_res in self.guessed.iter_mut().filter(|x| !x.is_guessed && x.ch==ch) {
            g_res.is_guessed = true;
        }
    }
}

struct GuessedRes {
    ch : char,
    is_guessed : bool,
}

impl GuessedRes {

    fn new(ch : char, is_guessed : bool) -> GuessedRes {
        GuessedRes {
            ch : ch,
            is_guessed : is_guessed,
        }
    }

    fn to_enum(&self) -> Guessed {
        if self.is_guessed {
            Guessed::Yes(self.ch)
        } else {
            Guessed::No
        }
    }
}

#[derive(Debug)]
pub enum Guessed {
    Yes(char),
    No,
}

#[derive(Debug)]
pub struct CurrentProgress {
    vec : Vec<Guessed>,
}

impl Display for CurrentProgress {
    fn fmt(&self, f : &mut Formatter) -> Result<(), Error> {
        let mut result = String::with_capacity(self.vec.len() * 2);
        for guess in self.vec.iter() {
            match *guess {
                Guessed::Yes(ch) => result.push(ch),
                Guessed::No => result.push('_'),
            }
            result.push(' ');
        }
        write!(f, "{}", result.as_slice())
    }
}
