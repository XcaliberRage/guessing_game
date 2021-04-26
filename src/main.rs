use ggez;
use glam; // Requires feature "mint"

use ggez::conf::{Conf, WindowSetup};
use std::process::exit;
use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color as Colour;
use ggez::event::{KeyCode, KeyMods};
use std::env;
use std::path;
use rand::*;

const MIN_NUM: i32 = 1;
const MAX_NUM: i32 = 100;
const MAX_LEN: usize = 3;

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

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;

        let title = graphics::Text::new(("Guess the number!", font, 48.0));

        let output = graphics::Text::new(("Ready? (Press Return to start)", font, 24.0));

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
            state
        };

        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG.into());

        let dest_point = glam::Vec2::new(0.0, 0.0);
        graphics::draw(ctx, &self.title, (dest_point, ))?;
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
        fn push_char(key: char, mut guess: &String) -> &String {

            if guess.chars().count() >= MAX_LEN {
                return guess;
            }
            guess.push(key);
            guess
        }

        let mut guess = &self.guess;
        let mut t: &String = guess;
        match Keypress::from_keycode(keycode) {
            Some(Keypress::Esc) => {
                println!("Thanks for playing!");
                exit(0)
            }
            Some(Keypress::Undo) => {
                self.guess.pop();
            }
            Some(Keypress::Ret) => {
                check_guess(&self.guess);
            }
            Some(Keypress::Negative) => {
                self.negative = true;
            }
            Some(Keypress::Zero) => { t = push_char('0', t);}
            Some(Keypress::One) => { t = push_char('1', t);}
            Some(Keypress::Two) => { t = push_char('2', t);}
            Some(Keypress::Three) => { t = push_char('3', t);}
            Some(Keypress::Four) => { t = push_char('4', t);}
            Some(Keypress::Five) => { t = push_char('5', t);}
            Some(Keypress::Six) => { t = push_char('6', t);}
            Some(Keypress::Seven) => { t = push_char('7', t);}
            Some(Keypress::Eight) => { t = push_char('8', t);}
            Some(Keypress::Nine) => { t = push_char('9', t); }
            _ => {}
        }
        let r = t;
        self.guess = *r;
    }
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


    let (ctx, events_loop) = ggez::ContextBuilder::new("guessing_game", "XcaliberRage")
        .add_resource_path(resource_dir)
        .window_setup(ggez::conf::WindowSetup::default()
            .title("A Rustically Guessing Game!"))
        .build()?;

    let state = GameState::new(&mut &ctx);

    ggez::event::run(ctx, events_loop, state);
}
