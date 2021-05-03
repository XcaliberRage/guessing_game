#![windows_subsystem = "windows"]
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
use std::fmt::{Display, Formatter, Result as fResult, Debug};

const DEFAULT_MIN_NUM: i32 = 1;
const DEFAULT_MAX_NUM: i32 = 100;
const DEFAULT_MAX_LEN: usize = 3;

struct Difficulty {
    min: i32,
    max: i32,
    len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Formats {
    title_size: f32,
    margin: f32,
    text_size: f32,
    err_size: f32,
    font: graphics::Font,
    left_gutter: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    NewGame,
    Guessing,
    Win,
    Settings,
}

#[derive(Debug)]
struct GameText {
    title: graphics::Text,
    output: graphics::Text,
    err: graphics::Text,
}


struct GameState {
    frames: usize,
    text: GameText,
    guess: String,
    negative: bool,
    secret_number: i32,
    guess_count: u32,
    state: State,
    formatting: Formats,
    difficulty: Difficulty,
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

impl Display for Keypress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "{}", self)
    }
}

impl Formats {
    pub fn new(ctx: &mut Context) -> Result<Formats, GameError> {

        Ok(
            Formats {
                title_size: 48.0,
                margin: 12.0,
                text_size: 24.0,
                err_size: 12.0,
                font: graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?,
                left_gutter: 10.0,
            })


    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "{}", self)
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult {
        write!(f, "State: {}", self.state)
    }
}

impl Display for GameText {
    fn fmt(&self, f: &mut Formatter<'_>) -> fResult { write!(f, "Text: {}", self) }
}

impl GameText {
    pub fn new() -> GameText {
        GameText {
            title: graphics::Text::default(),
            output: graphics::Text::default(),
            err: graphics::Text::default(),
        }
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let s = GameState {
            frames: 0,
            text: GameText::new(),
            secret_number: 0,
            guess: String::from(""),
            negative: false,
            guess_count: 0,
            state: State::NewGame,
            formatting: Formats::new(ctx).unwrap(),
            difficulty: Difficulty{
                min: DEFAULT_MIN_NUM,
                max: DEFAULT_MAX_NUM + 1,
                len: DEFAULT_MAX_LEN
            }
        };
        Ok(s)
    }

    // Set the default output for a new game
    pub fn main_menu(&mut self) {
        self.text.title = self.textify("Guess the number!".to_string(), self.formatting.font, self.formatting.title_size);
        self.text.output = self.textify("Ready? (Press Return to start)".to_string(), self.formatting.font, self.formatting.text_size);
        self.text.err = self.textify("Press Esc to quit".to_string(), self.formatting.font, self.formatting.err_size);
        self.reset_guesses();
    }

    pub fn reset_guesses(&mut self) {
        self.guess_count = 0;
        self.negative = false;
        self.guess = "0".to_string();
    }

    // Gets a text from a string using the given font and size
    fn textify(&self, some_text: String, font: graphics::Font, size: f32) -> graphics::Text {
        graphics::Text::new((some_text, font, size))
    }

    // Call this to change the game state
    fn new_state(&mut self, some_state: State) -> State {

        match some_state {
            State::Guessing => {
                let guess = "0";
                self.guess = String::from(guess);
                self.negative = false;
                self.text.output = self.textify(String::from(guess), self.formatting.font, self.formatting.text_size);
                self.text.title = self.textify("Type your guess below!".to_string(), self.formatting.font, self.formatting.title_size);
            }
            State::Win => {
                self.text.title = self.textify(format!("The number was {}.", self.secret_number), self.formatting.font, self.formatting.title_size);
                self.text.output = self.textify(format!("You won in {} guesses! Press Return to go back to the main menu", self.guess_count), self.formatting.font, self.formatting.text_size);
            }
            State::NewGame => {
                self.main_menu();
                self.secret_number = rand::thread_rng().gen_range(self.difficulty.min, self.difficulty.max);
            }
            _ => {}
        }

        some_state

    }

    // parses the current guess and checks if it is correct or not, giving a clue if not
    pub fn check_guess(&mut self) -> bool {

        self.guess_count += 1;
        let mut guess = self.guess.clone().parse::<i32>().unwrap();
        if self.negative {
            guess *= -1;
        }

        if guess == self.secret_number {
            return true
        }

        let mut deviance = "higher";

        if guess > self.secret_number {
            deviance = "lower";
        }

        self.text.title = self.textify(
            format!("You guessed {}, try {}!", guess_string_compiler(
                &self.guess, self.negative), deviance),
            self.formatting.font,
            self.formatting.title_size
        );
        false
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        match self.state {
            State::NewGame => self.main_menu(),
            State::Guessing => {
                self.text.output = self.textify(guess_string_compiler(
                    &self.guess, self.negative), self.formatting.font , self.formatting.text_size);
            },
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG.into());

        let title_point = glam::Vec2::new(self.formatting.left_gutter, 0.0);
        let guess_point = glam::Vec2::new(self.formatting.left_gutter, self.formatting.title_size + self.formatting.margin);
        let err_point = glam::Vec2::new(self.formatting.left_gutter, self.formatting.title_size + (self.formatting.margin * 2.0) + self.formatting.text_size);

        graphics::draw(ctx, &self.text.title, (title_point, ))?;
        graphics::draw(ctx, &self.text.output, (guess_point, ))?;
        graphics::draw(ctx, &self.text.err, (err_point, ))?;

        graphics::present(ctx)?;

        self.frames += 1;
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
        // Replaces leading zeros too
        fn push_char( key: char, game: &mut GameState) {

            let guess_val = guess_string_compiler(
                &format!("{}{}", &game.guess.clone(), key)
                , game.negative).parse::<i32>().unwrap();

            if game.guess.chars().count() >= game.difficulty.len ||
                guess_val < game.difficulty.min ||
                guess_val > game.difficulty.max
            {
                return
            }

            game.guess.push(key);
            game.guess = game.guess.clone().parse::<u32>().unwrap().to_string();
        }

        match Keypress::from_keycode(keycode) {
            Some(Keypress::Esc) => {
                println!("Thanks for playing!");
                exit(0)
            }
            Some(Keypress::Undo) => {
                self.guess.pop();
            }
            Some(Keypress::Ret) => {
                match self.state {
                    State::Guessing => {
                        if !self.check_guess() {
                            self.guess = "0".to_string();
                            self.negative = false;
                        } else {
                            self.state = self.new_state(State::Win);
                        }
                    }
                    State::NewGame => {
                        self.state = self.new_state(State::Guessing); }
                    State::Win => {
                        self.state = self.new_state(State::NewGame);
                    }
                    _ => {}
                }
            }
            Some(Keypress::Negative) => {
                self.negative = if self.difficulty.min < 0 {!self.negative} else {self.negative};
            }
            Some(Keypress::Zero) => push_char('0', self),
            Some(Keypress::One) => push_char('1', self),
            Some(Keypress::Two) => push_char('2', self),
            Some(Keypress::Three) => push_char('3', self),
            Some(Keypress::Four) => push_char('4', self),
            Some(Keypress::Five) => push_char('5', self),
            Some(Keypress::Six) => push_char('6', self),
            Some(Keypress::Seven) => push_char('7',  self),
            Some(Keypress::Eight) => push_char('8',  self),
            Some(Keypress::Nine) => push_char('9',  self),
            _ => {}
        }

        self.guess = match self.state {
            State::Guessing => self.guess.clone().parse().unwrap(),
            State::Win => self.guess.clone().parse().unwrap(),
            _ => String::from("0"),
        };
    }
}

// Shoves a minus on the front of the number
fn guess_string_compiler(value: &String, is_negative: bool) -> String {
    let mut return_val = if is_negative {"-"} else {""}
        .to_string();
    return_val.push_str(value);
    return_val
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

    let (mut ctx, events_loop) = ggez::ContextBuilder::new("guessing_game", "XcaliberRage")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default()
            .title("A Rustically Guessing Game!"))
        .build()?;

    let mut state = GameState::new(&mut ctx)?;
    state.new_state(State::NewGame);

    ggez::event::run(ctx, events_loop, state);
}
