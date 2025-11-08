extern crate cosmic;
mod message;
mod state;
mod totp_subscription;
mod update;
mod view;
use cosmic::iced;
// use cosmic::iced::Theme;
use message::Message;
use state::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = cosmic::app::Settings::default();
    let args = vec![(state::Content::Oath, "Solo2 GUI".to_string())];
    cosmic::app::run::<State>(settings, args)?;

    Ok(())
    // iced::application("Solo2 GUI", State::update, State::view)
    // .theme(|_| Theme::Dark)
    // .subscription(State::totp_subscription)
    // .init()
}
