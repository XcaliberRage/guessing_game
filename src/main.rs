use ggez;
use glam; // Requires feature "mint"

use ggez::conf::{Conf, WindowSetup};
use std::process::exit;
use ggez::{event, graphics, Context, GameResult, GameError};
use ggez::graphics::Color as Colour;
use ggez::event::{KeyCode, KeyMods};
use std::env;
use std::path;
use rand::*;

const MIN_NUM: i32 = 1;
const MAX_NUM: i32 = 100;
const MAX_LEN: usize = 3;

pub struct Formats {
    title_size: f32,
    margin: f32,
    text_size: f32,
    font: graphics::Font
}

enum State {
    NewGame,
    Guessing,
    Win,
}

struct GameState {
    frames: usize,
    title: graphics::Text,
    output: graphics::Text,
    guess: String,
    negative: bool,
    secret_number: i32,
    guess_count: u32,
    state: State,
    formatting: Formats,
}

pub const BG: Colour = Colour {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Keypress {
    Esc,
    Ret,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Undo,
    Negative,
}

impl Keypress {
    pub fn from_keycode(key: KeyCode) -> Option<Keypress> {
        match key {
            KeyCode::Return => Some(Keypress::Ret),
            KeyCode::Escape => Some(Keypress::Esc),
            KeyCode::Back => Some(Keypress::Undo),
            KeyCode::Key0 => Some(Keypress::Zero),
            KeyCode::Key1 => Some(Keypress::One),
            KeyCode::Key2 => Some(Keypress::Two),
            KeyCode::Key3 => Some(Keypress::Three),
            KeyCode::Key4 => Some(Keypress::Four),
            KeyCode::Key5 => Some(Keypress::Five),
            KeyCode::Key6 => Some(Keypress::Six),
            KeyCode::Key7 => Some(Keypress::Seven),
            KeyCode::Key8 => Some(Keypress::Eight),
            KeyCode::Key9 => Some(Keypress::Nine),
            KeyCode::Minus => Some(Keypress::Negative),
            _ => None,
        }
    }
}

impl Formats {
    pub fn new(ctx: &'static mut Context) -> Result<Formats, GameError> {

        Ok(
            Formats {
                title_size: 48.0,
                margin: 12.0,
                text_size: 24.0,
                font: graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?,
            })


    }
}

impl GameState {
    pub fn new(&self, ctx: &'static mut Context) -> GameResult<GameState> {

        let formatting = Formats::new(ctx).unwrap();
        let title = self.textify("Guess the number!".to_string(), self.formatting.font, self.formatting.title_size);
        let output = self.textify("Ready? (Press Return to start)".to_string(), self.formatting.font, self.formatting.text_size);

        let guess = String::from("");
        let secret_number = rand::thread_rng().gen_range(MIN_NUM, MAX_NUM+1);
        let guess_count = 0;
        let state = State::NewGame;

        let s = GameState {
            frames: 0,
            title,
            output,
            secret_number,
            guess,
            negative: false,
            guess_count,
            state,
            formatting,
        };

        Ok(s)
    }

    fn textify(&self, some_text: String, font: graphics::Font, size: f32) -> graphics::Text {
        graphics::Text::new((some_text, self.formatting.font, self.formatting.title_size))
    }

    fn new_state(&mut self, some_state: Option<State>) -> Option<State> {

        match some_state {
            Some(State::Guessing) => {
                self.guess = String::from("0");
                self.negative = false;
                self.output = self.textify(self.guess, self.formatting.font, self.formatting.text_size);
            }
            Some(State::Win) => {
                self.title = self.textify(format!("The number was {}.", self.secret_number), self.formatting.font, self.formatting.title_size);
                self.output = self.textify(format!("You won in {} guesses!", self.guess_count), self.formatting.font, self.formatting.text_size);
            }
            _ => {}
        }

        some_state

    }

    // Parses the current guess and returns true if it is the correct secret number.
    fn correct_guess(&self) -> bool {

        let mut guess_as_int = self.guess.parse::<i32>().unwrap();
        guess_as_int = if self.negative {guess_as_int * -1} else {guess_as_int};

        guess_as_int == self.secret_number
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG.into());

        let title_point = glam::Vec2::new(0.0, 0.0);
        let guess_point = glam::Vec2::new(0.0, (self.formatting.title_size + self.formatting.margin));
        graphics::draw(ctx, &self.title, (title_point, ))?;

        graphics::draw(ctx, &self.output, (guess_point, ))?;

        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {

        // Helper function to stick a character on a string provided the value is not bigger than max
        fn push_char<'a>(key: char, mut guess: &'a mut String) -> &'a mut String {

            if guess.chars().count() >= MAX_LEN {
                return guess;
            }
            guess.push(key);
            guess
        }

        let mut guess = &mut self.guess;
        match Keypress::from_keycode(keycode) {
            Some(Keypress::Esc) => {
                println!("Thanks for playing!");
                exit(0)
            }
            Some(Keypress::Undo) => {
                guess.pop();
            }
            Some(Keypress::Ret) => {
                if self.state == State::Guessing {
                    check_guess(&guess);
                } else if self.correct_guess() {
                    self.new_state(Some(State::Win));
                } else {
                    self.new_state(Some(State::Guessing));
                }
            }
            Some(Keypress::Negative) => {
                self.negative = true;
            }
            Some(Keypress::Zero) => { guess = push_char('0', guess);}
            Some(Keypress::One) => { guess = push_char('1', guess);}
            Some(Keypress::Two) => { guess = push_char('2', guess);}
            Some(Keypress::Three) => { guess = push_char('3', guess);}
            Some(Keypress::Four) => { guess = push_char('4', guess);}
            Some(Keypress::Five) => { guess = push_char('5', guess);}
            Some(Keypress::Six) => { guess = push_char('6', guess);}
            Some(Keypress::Seven) => { guess = push_char('7', guess);}
            Some(Keypress::Eight) => { guess = push_char('8', guess);}
            Some(Keypress::Nine) => { guess = push_char('9', guess); }
            _ => {}
        }
        self.guess = if self.state == State::Guessing {(*guess.clone()).parse().unwrap()} else {"0"};
    }
}

fn guess_string_compiler(value: &String, is_negative: bool) -> String {
    let mut r_val = if is_negative {"-"} else {""}
        .to_string();
    r_val.push_str(value);
    r_val
}

pub fn check_guess(guess: &String) {
    println!("You have guessed {}", *guess);
}

pub fn main() -> GameResult {
    //println!("Guess the number!");

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut num_guesses = 0u32;

    /*loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        num_guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                println!("You won in {} guesses!", num_guesses);
                break;
            }
        }
    }*/


    let (mut ctx, events_loop) = ggez::ContextBuilder::new("guessing_game", "XcaliberRage")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default()
            .title("A Rustically Guessing Game!"))
        .build()?;

    let state = GameState::new(&mut ctx)?;

    ggez::event::run(ctx, events_loop, state);
}
