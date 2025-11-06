mod admin_content;
mod totp_content;
use crate::message::Message;
use crate::state::{Content, Pane, State};
extern crate iced;
extern crate solo2;
use iced::{
    Element, Fill,
    widget::{self, button, container, pane_grid, text},
};

pub const SPACING: u16 = 10;

impl State {
    // Code concerned with displaying the state and returning messages (showing UI and responding to interaction)
    pub fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |pane, _, _| {
            // Option<&Pane> will always return because we get pane from &self.panes
            // Add content to AppList and Content panes
            pane_grid::Content::new(match &self.panes.get(pane).unwrap() {
                Pane::AppList => container(widget::column![
                    button(text("Oath").size(20).width(Fill))
                        .style(button::secondary)
                        .on_press(Message::OathButtonPress)
                        .width(Fill),
                    button(text("Admin").size(20).width(Fill))
                        .style(button::secondary)
                        .on_press(Message::AdminButtonPress)
                        .width(Fill),
                ])
                .style(container::rounded_box)
                .into(),
                Pane::Content => {
                    match &self.content {
                        // Content for Oath app
                        Content::Oath => totp_content::draw_totp_content(self),
                        Content::Admin => admin_content::draw_admin_content(self),
                    }
                }
            })
        });
        container(pane_grid.spacing(SPACING))
            .width(Fill)
            .height(Fill)
            .padding(10)
            .into()
    }
}
