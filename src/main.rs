extern crate iced;
mod message;
mod state;
mod totp_subscription;
mod update;
mod view;
use iced::Theme;
use iced::time::{self, Duration};
use message::Message;
use state::State;

fn main() -> iced::Result {
    iced::application("Solo2 GUI", State::update, State::view)
        .theme(|_| Theme::SolarizedDark)
        .subscription(State::totp_subscription)
        .run()
}
