use crate::State;
extern crate iced;
use crate::Message;
use crate::view::SPACING;
use iced::Shrink;
use iced::widget::{self, button, text};

pub fn draw_admin_content<'a>(state: &'a State) -> iced::Element<'a, Message> {
    let uuid_text: iced::Element<Message> =
        widget::row![text("UUID:"), text(&state.admin_state.uuid)]
            .spacing(SPACING)
            .into();
    let version: iced::Element<Message> =
        widget::row![text("Firmware Version:"), text(&state.admin_state.version)]
            .spacing(SPACING)
            .into();
    let locked: iced::Element<Message> =
        widget::row![text("Locked:"), text(state.admin_state.locked)]
            .spacing(SPACING)
            .into();
    let wink_button: iced::Element<Message> =
        button("Wink").width(Shrink).on_press(Message::Wink).into();

    return widget::column![version, uuid_text, locked, wink_button]
        .spacing(SPACING)
        .into();
}
