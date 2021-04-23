use ggez;
use glam; // Requires feature "mint"

use ggez::conf::{Conf, WindowSetup};
use std::process::exit;
use ggez::graphics::Color as Colour;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use std::env;
use std::path;
use rand::*;

const MIN_NUM: i32 = 1;
const MAX_NUM: i32 = 100;

enum State {
    NewGame,
    Guessing,
    Win,
}

struct MainState {
    frames: usize,
    title: graphics::Text,
    output: graphics::Text,
    guess: u32,
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
}

impl Keypress {
    pub fn from_keycode(key: KeyCode) -> Option<Keypress> {
        match key {
            KeyCode::Return => Some(Keypress::Ret),
            KeyCode::Escape => Some(Keypress::Esc),
            KeyCode::Backspace => Some(Keypress::Undo),
            KeyCode::Numpad0 => Some(Keypress::Zero),
            KeyCode::Numpad1 => Some(Keypress::One),
            KeyCode::Numpad2 => Some(Keypress::Two),
            KeyCode::Numpad3 => Some(Keypress::Three),
            KeyCode::Numpad4 => Some(Keypress::Four),
            KeyCode::Numpad5 => Some(Keypress::Five),
            KeyCode::Numpad6 => Some(Keypress::Six),
            KeyCode::Numpad7 => Some(Keypress::Seven),
            KeyCode::Numpad8 => Some(Keypress::Eight),
            KeyCode::Numpad9 => Some(Keypress::Nine),
            _ => None,
        }
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;

        let title = graphics::Text::new(("Guess the number!", font, 48.0));

        let output = graphics::Text::new(("Ready? (Press Return to start)", font, 24));

        let guess = 0;
        let secret_number = rand::thread_rng().gen_range(MIN_NUM, MAX_NUM+1);
        let guess_count = 0;
        let state = State::NewGame;

        let s = MainState {
            frames: 0,
            title,
            output,
            secret_number,
            guess,
            guess_count,
            state
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, BG.into());

        let dest_point = glam::Vec2::new(0.0, 0.0);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            keycode: KeyCode,
            _keymod: KeyMods,
            _repeat: bool,
        ) {

        }

        Ok(())
    }
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


    let cb = ggez::ContextBuilder::new("guessing_game", "XcaliberRage").add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
