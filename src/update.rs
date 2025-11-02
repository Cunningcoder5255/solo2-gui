use crate::message::Message;
use crate::state::State;
extern crate solo2;
use crate::state::Content;
use solo2::{Select, UuidSelectable, apps::Oath};

impl State {
    pub fn update(state: &mut State, message: Message) -> iced::Task<Message> {
        match message {
            Message::OathButtonPress => {
                state.content = Content::Oath;
                iced::Task::none()
            }
            Message::TOTPLabelPress(label) => {
                let mut devices = solo2::Device::list();
                if devices.len() != 0 {
                    // Early return to prevent accessing empty vec
                    return iced::Task::none();
                }
                // Convert from Device type to Solo2 type
                let mut solo2 = devices
                    .swap_remove(0)
                    .into_solo2()
                    .expect("Device is not a solo2 device.");
                let mut app = Oath::select(&mut solo2).expect("Could not enter oath app.");
                iced::clipboard::write::<Message>(
                    app.authenticate(solo2::apps::oath::Authenticate::with_label(&label))
                        .expect("No TOTP with label: {label}"),
                )
            }
        }
    }
}
