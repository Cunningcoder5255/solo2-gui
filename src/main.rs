extern crate iced;
mod message;
mod state;
mod update;
mod view;
use state::State;

fn main() -> iced::Result {
    iced::run("Solo2 GUI", State::update, State::view)
}
