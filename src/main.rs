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

const MIN_NUM: i32 = 1;
const MAX_NUM: i32 = 100;
const MAX_LEN: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Formats {
    title_size: f32,
    margin: f32,
    text_size: f32,
    font: graphics::Font
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    NewGame,
    Guessing,
    Win,
}

struct GameText {
    title: graphics::Text,
    output: graphics::Text,
    err: graphics::Text,
}

#[derive(Clone, Debug)]
struct GameState {
    frames: usize,
    text: GameText,
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
                font: graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Title: {}\n"), self.title)
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let s = GameState {
            frames: 0,
            title: graphics::Text::default(),
            output: graphics::Text::default(),
            secret_number: rand::thread_rng().gen_range(MIN_NUM, MAX_NUM+1),
            guess: String::from(""),
            negative: false,
            guess_count: 0,
            state: State::NewGame,
            formatting: Formats::new(ctx).unwrap(),
        };
        Ok(s)
    }

    // Set the default output for a new game
    pub fn main_menu(&mut self) {
        self.title = self.textify("Guess the number!".to_string(), self.formatting.font, self.formatting.title_size);
        self.output = self.textify("Ready? (Press Return to start)".to_string(), self.formatting.font, self.formatting.text_size);
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
                self.output = self.textify(String::from(guess), self.formatting.font, self.formatting.text_size);
                self.title = self.textify("Type your guess below!".to_string(), self.formatting.font, self.formatting.title_size);
            }
            State::Win => {
                self.title = self.textify(format!("The number was {}.", self.secret_number), self.formatting.font, self.formatting.title_size);
                self.output = self.textify(format!("You won in {} guesses!", self.guess_count), self.formatting.font, self.formatting.text_size);
            }
            State::NewGame => {
                self.main_menu();
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

    pub fn check_guess(&mut self) {
        self.title = self.textify(
            format!("You guessed {}, try again!", guess_string_compiler(
                &self.guess, self.negative)),
            self.formatting.font,
            self.formatting.title_size
        );
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        match self.state {
            State::NewGame => self.main_menu(),
            State::Guessing => {
                self.output = self.textify(guess_string_compiler(&self.guess, self.negative), self.formatting.font , self.formatting.text_size);
            },
            _ => {}
        }

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
            dbg!(self.state);
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
        println!("key_down_event: {:?}", keycode);

        // Helper function to stick a character on a string provided the value is not bigger than max
        // Replaces leading zeros too
        pub fn push_char( key: char, game: &mut GameState) {
            let guess_val = guess_string_compiler(
                &format!("{}{}", &game.guess.clone(), key)
                , game.negative).parse::<i32>().unwrap();
            println!("{}", guess_val);
            if game.guess.chars().count() >= MAX_LEN ||
                guess_val < MIN_NUM || guess_val > MAX_NUM
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
                        self.check_guess();
                        self.guess = "0".to_string();
                        self.negative = false;
                    }
                    State::NewGame => {
                        self.state = self.new_state(State::Guessing); }
                    State::Win => {
                        self.state = self.new_state(State::NewGame);
                        self.state = self.new_state(State::Guessing);
                    }
                }
            }
            Some(Keypress::Negative) => {
                self.negative = !self.negative;
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
            State::Guessing => (self.guess.clone()).parse().unwrap(),
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

    let mut state = GameState::new(&mut ctx)?;
    state.new_state(State::NewGame);

    ggez::event::run(ctx, events_loop, state);
}
