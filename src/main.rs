use calculator::{Calculator, State};
use iced::{
    keyboard::{self, Key, Modifiers},
    Size, Subscription, Theme,
};

fn theme(_: &Calculator) -> Theme {
    Theme::Ferra
}

fn display_key_press(key: Key, _: Modifiers) -> Option<State> {
    match key {
        Key::Character(character) => {
            if character == String::from("+") {
                return Some(State::Plus);
            }

            if character == String::from("-") {
                return Some(State::Minus);
            }

            if character == String::from("x") {
                return Some(State::Multiply);
            }

            if character == String::from(".") || character == String::from(",") {
                return Some(State::Comma);
            }
            let parsed = character.parse::<f64>();
            if let Ok(value) = parsed {
                return Some(State::Value(value));
            }
            return Some(State::None);
        }

        Key::Named(named) => match named {
            keyboard::key::Named::Backspace => return Some(State::Remove),
            keyboard::key::Named::Enter => return Some(State::Equal),
            _ => {}
        },
        Key::Unidentified => println!("UFO"),
    }
    Some(State::None)
}

fn subscription(_: &Calculator) -> Subscription<State> {
    keyboard::on_key_press(display_key_press)
}

fn main() -> iced::Result {
    iced::application("Calculator", Calculator::update, Calculator::view)
        .theme(theme)
        .subscription(subscription)
        .window_size(Size {
            width: 500 as f32,
            height: 600 as f32,
        })
        .run()
}
