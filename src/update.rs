use crate::message::Message;
use crate::state::State;
extern crate solo2;
use crate::state::Content;
use solo2::{Select, UuidSelectable, apps::Oath};

impl State {
    pub fn update(state: &mut State, message: Message) -> iced::Task<Message> {
        match message {
            Message::OathButtonPress => {
                state.content = Content::Oath(state.adding_totp);
                iced::Task::none()
            }
            Message::CancelAddingTOTP => {
                state.adding_totp = false;
                iced::Task::none()
            }
            Message::UpdateLabelInput(label) => {
                state.label_input = label;
                iced::Task::none()
            }
            Message::UpdateSecretInput(secret) => {
                state.secret_input = secret;
                iced::Task::none()
            }
            Message::OathTOTPLifeRefresh(_instant) => {
                state.content = Content::Oath(state.adding_totp);
                iced::Task::none()
            }
            Message::AddTOTP => {
                let mut devices = solo2::Device::list();
                if devices.len() == 0 {
                    // Early return to prevent accessing empty vec
                    return iced::Task::none();
                }
                // Convert from Device type to Solo2 type
                let mut solo2 = devices
                    .swap_remove(0)
                    .into_solo2()
                    .expect("Device is not a solo2 device.");
                let mut app = Oath::select(&mut solo2).expect("Could not enter oath app.");

                app.register(
                    solo2::apps::oath::Credential::default_totp(
                        &state.label_input,
                        &state.secret_input,
                    )
                    .expect("Could not get credential"),
                )
                .expect("Could not register TOTP code.");
                state.adding_totp = false;
                iced::Task::none()
            }
            Message::AddTOTPScreen => {
                state.adding_totp = true;
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
