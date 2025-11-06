use crate::Message;
use crate::State;
use std::time::SystemTime;
extern crate iced;
use crate::view::SPACING;
use iced::Fill;
use iced::Shrink;
use iced::alignment;
use iced::widget::{
    self, button, center, container, horizontal_rule, row, scrollable, svg, text, text_input,
};

pub fn draw_totp_content<'a>(state: &'a State) -> iced::Element<'a, Message> {
    let copy_svg = svg::Handle::from_memory(include_bytes!("../../svg/reload.svg").as_slice());
    if state.solo2.is_none() {
        return row![
            text("No solo2 device.").size(32),
            button(svg(copy_svg).width(32).height(32))
                .width(Shrink)
                .on_press(Message::ReloadDevices)
        ]
        .spacing(SPACING)
        .into();
    }
    // Vector to push elements to
    let mut oath_labels: Vec<iced::Element<Message>> = vec![];

    let totp_list = &state.oath_state.totp_list;

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
        let totp_text: iced::Element<Message> =
            center(text(totp_code).size(32)).width(Shrink).into();
        let totp_lifetime_bar = widget::progress_bar(0.00..=30.0, totp_lifetime)
            .width(60)
            .height(Fill);
        let totp_lifetime_text = text(totp_lifetime)
            .width(60)
            .size(24)
            .height(Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center);
        // Source - https://stackoverflow.com/a
        // Posted by Jack Baldwin
        // Retrieved 2025-11-06, License - CC BY-SA 4.0
        // Include svg into application

        let copy_svg = svg::Handle::from_memory(include_bytes!("../../svg/copy.svg").as_slice());

        let totp_copy_button: iced::Element<Message> = button(svg(copy_svg).width(Shrink))
            .width(Shrink)
            .on_press(Message::CopyTOTP(label.clone()))
            .style(button::secondary)
            .into();
        let delete_button: iced::Element<Message> = button(
            text("Delete")
                .align_x(alignment::Horizontal::Center)
                .width(Fill),
        )
        .on_press(Message::DeleteTOTP(label.clone()))
        .width(Fill)
        .style(button::danger)
        .into();

        let mut totp_widget: iced::Element<Message> = button(
            row![
                totp_label,
                totp_copy_button,
                totp_text,
                widget::stack![totp_lifetime_bar, totp_lifetime_text]
                    .width(Shrink)
                    .height(Fill)
            ]
            .spacing(SPACING)
            .height(Shrink),
        )
        .on_press(Message::TOTPLabelPress(label.clone()))
        .style(button::secondary)
        .padding(10)
        .into();

        if state.oath_state.deleting_totp == *label {
            totp_widget = container(iced::widget::column![totp_widget, delete_button])
                .style(container::rounded_box)
                .into();
        }
        oath_labels.push(totp_widget);
    }

    // Add horizontal rule to separate add button/add screen
    oath_labels.push(horizontal_rule(4).into());
    // Draw content for adding a TOTP code
    if state.oath_state.adding_totp {
        let label_input: iced::Element<Message> =
            text_input("Label", &state.oath_state.label_input)
                .on_input(Message::UpdateLabelInput)
                .into();
        let secret_input: iced::Element<Message> =
            text_input("Secret Code", &state.oath_state.secret_input)
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
        let invalid_secret_text: iced::Element<Message> = text("Secret is not 32 characters.")
            .align_x(alignment::Horizontal::Center)
            .into();
        let mut adding_totp_widgets = iced::widget::column![
            row![label_input, secret_input].spacing(SPACING),
            row![cancel_button, add_button].spacing(SPACING)
        ]
        .spacing(SPACING)
        .into();

        // Add invalid totp code length error message if necessary
        if state.oath_state.invalid_totp_code_length {
            adding_totp_widgets =
                iced::widget::column![adding_totp_widgets, invalid_secret_text].into();
        }
        oath_labels.push(adding_totp_widgets);
        // Draw "+" button to start adding a TOTP code
    } else {
        let add_totp_button: iced::Element<Message> =
            button(center(text("+").size(32)).height(Shrink))
                .on_press(Message::AddTOTPScreen)
                .width(Fill)
                .style(button::secondary)
                .padding(10)
                .into();
        oath_labels.push(add_totp_button);
    }
    scrollable(
        iced::widget::Column::with_children(oath_labels)
            .spacing(SPACING)
            .width(Fill),
    )
    .into()
}
