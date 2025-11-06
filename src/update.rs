use crate::message::Message;
use crate::state::State;
extern crate solo2;
use crate::state::Content;
use solo2::{Select, apps::Oath};

impl State {
    pub fn update(state: &mut State, message: Message) -> iced::Task<Message> {
        match message {
            Message::OathButtonPress => {
                state.content = Content::Oath;
                iced::Task::none()
            }
            Message::CancelAddingTOTP => {
                state.oath_state.adding_totp = false;
                state.oath_state.invalid_totp_code_length = false;
                state.oath_state.label_input = "".to_string();
                state.oath_state.secret_input = "".to_string();
                iced::Task::none()
            }
            Message::UpdateLabelInput(label) => {
                state.oath_state.label_input = label;
                iced::Task::none()
            }
            Message::UpdateSecretInput(secret) => {
                state.oath_state.secret_input = secret;
                iced::Task::none()
            }
            Message::ReloadDevices => {
                state.update_devices();
                iced::Task::none()
            }
            Message::OathTOTPLifeRefresh(_instant) => {
                state.content = Content::Oath;
                iced::Task::none()
            }
            Message::AddTOTP => {
                if state.oath_state.secret_input.len() != 16 {
                    state.oath_state.invalid_totp_code_length = true;
                    return iced::Task::none();
                }
                state.oath_state.invalid_totp_code_length = false;
                let solo2 = state.solo2.as_mut().unwrap(); // Can unwrap because totp screen won't be shown if there are no devices
                let mut app = Oath::select(solo2).expect("Could not enter oath app.");

                app.register(
                    solo2::apps::oath::Credential::default_totp(
                        &state.oath_state.label_input,
                        &state.oath_state.secret_input,
                    )
                    .expect("Could not get credential"),
                )
                .expect("Could not register TOTP code.");
                // Clear inputs and get out of adding_totp screen
                state.oath_state.label_input = "".to_string();
                state.oath_state.secret_input = "".to_string();
                state.oath_state.adding_totp = false;
                state.update_devices();
                iced::Task::none()
            }
            Message::AddTOTPScreen => {
                state.oath_state.deleting_totp = "".to_string();
                state.oath_state.adding_totp = true;
                iced::Task::none()
            }
            Message::CopyTOTP(label) => {
                let solo2 = state.solo2.as_mut().unwrap();
                let mut app = Oath::select(solo2).expect("Could not enter oath app.");
                iced::clipboard::write::<Message>(
                    app.authenticate(solo2::apps::oath::Authenticate::with_label(&label))
                        .expect("No TOTP with label: {label}"),
                )
            }
            Message::TOTPLabelPress(label) => {
                // Functionality to toggle deleting totp button
                if state.oath_state.deleting_totp == label {
                    state.oath_state.deleting_totp = "".to_string();
                } else {
                    state.oath_state.deleting_totp = label;
                }
                iced::Task::none()
            }
            Message::DeleteTOTP(label) => {
                let solo2 = state.solo2.as_mut().unwrap();
                let mut app = Oath::select(solo2).expect("Could not enter oath app.");
                app.delete(label).expect("Could not delete TOTP.");
                state.oath_state.secret_input = "".to_string();
                state.oath_state.label_input = "".to_string();
                state.update_devices();
                iced::Task::none()
            }
        }
    }
}
