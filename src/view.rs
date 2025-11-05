use crate::message::Message;
use crate::state::{Content, Pane, State};
use std::time::SystemTime;
extern crate iced;
extern crate solo2;
use iced::{
    Element, Fill, Shrink, alignment,
    widget::{self, button, center, container, pane_grid, row, text, text_input},
};

impl State {
    // Code concerned with displaying the state and returning messages (showing UI and responding to interaction)
    pub fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |pane, _, _| {
            // Option<&Pane> will always return because we get pane from &self.panes
            // Add content to AppList and Content panes
            pane_grid::Content::new(match &self.panes.get(pane).unwrap() {
                Pane::AppList => container(
                    button(text("Oath").size(20).width(Fill))
                        .style(button::secondary)
                        .on_press(Message::OathButtonPress)
                        .width(Fill),
                )
                .style(container::rounded_box)
                .into(),
                Pane::Content => {
                    match &self.content {
                        // Content for Oath app
                        Content::Oath => draw_totp_content(self),
                    }
                }
            })
        });
        container(pane_grid.spacing(10))
            .width(Fill)
            .height(Fill)
            .padding(10)
            .into()
    }
}

fn draw_totp_content<'a>(state: &'a State) -> iced::Element<'a, Message> {
    if state.solo2.is_none() {
        return text("No solo2 device.").into();
    }
    // Vector to push elements to
    let mut oath_labels: Vec<iced::Element<Message>> = vec![];

    let totp_list = &state.totp_list;

    // How much time a totp code has left before expiring
    let totp_lifetime = (30
        - (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time before unix epoch somehow.")
            .as_secs()
            % 30)) as f32;

    // Add TOTP labels and codes to vector to be turned added to a column and drawn
    for (label, totp_code) in totp_list.into_iter() {
        let totp_label = text(label.clone())
            .width(Fill)
            .height(Fill)
            .size(24)
            .align_y(alignment::Vertical::Center);
        let totp_text: iced::Element<Message> = center(text(totp_code).size(32))
            .width(Shrink)
            .height(Shrink)
            .into();
        let totp_lifetime_bar = widget::progress_bar(0.00..=30.0, totp_lifetime)
            .width(40)
            .height(Fill);
        let totp_lifetime_text = text(totp_lifetime)
            .width(40)
            .height(Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center);
        oath_labels.push(
            button(
                row![
                    totp_label,
                    totp_text,
                    widget::stack![totp_lifetime_bar, totp_lifetime_text]
                        .width(Shrink)
                        .height(Fill)
                ]
                .spacing(10)
                .height(Shrink),
            )
            .on_press(Message::TOTPLabelPress(label.clone()))
            .style(button::secondary)
            .padding(10)
            .into(),
        );
    }
    if state.adding_totp {
        let label_input: iced::Element<Message> = text_input("Label", &state.label_input)
            .on_input(Message::UpdateLabelInput)
            .into();
        let secret_input: iced::Element<Message> = text_input("Secret Code", &state.secret_input)
            .on_input(Message::UpdateSecretInput)
            .into();
        let add_button: iced::Element<Message> = button(center("Add Code").height(Shrink))
            .on_press(Message::AddTOTP)
            .width(Fill)
            .into();
        let cancel_button: iced::Element<Message> = button(center("Cancel").height(Shrink))
            .on_press(Message::CancelAddingTOTP)
            .width(Fill)
            .style(button::secondary)
            .into();
        oath_labels.push(
            iced::widget::column![
                row![label_input, secret_input].spacing(10),
                row![cancel_button, add_button].spacing(10)
            ]
            .spacing(10)
            .into(),
        );
    } else {
        let add_totp_button = button(center(text("+").size(32)).height(Shrink))
            .on_press(Message::AddTOTPScreen)
            .width(Fill)
            .style(button::secondary)
            .padding(10)
            .into();
        oath_labels.push(add_totp_button);
    }
    iced::widget::Column::with_children(oath_labels)
        .spacing(10)
        .width(Fill)
        .into()
}
