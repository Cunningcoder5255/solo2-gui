use crate::Message;
use crate::State;
use iced::{
    Subscription,
    time::{self, Duration},
};

impl State {
    pub fn totp_subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(Message::OathTOTPLifeRefresh)
    }
}
