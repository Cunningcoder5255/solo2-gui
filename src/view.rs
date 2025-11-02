use crate::message::Message;
use crate::state::{Content, Pane, State};
use std::time::SystemTime;
extern crate solo2;
use solo2::apps::{Oath, oath};
use solo2::{Select, UuidSelectable};
extern crate iced;
use iced::{
    Element, Fill, Shrink, alignment,
    widget::{self, button, container, pane_grid, row, text},
};

impl State {
    // Code concerned with displaying the state and returning messages (showing UI and responding to interaction)
    pub fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |pane, _, _| {
            // Option<&Pane> will always return because we get pane from &self.panes
            // Add content to AppList and Content panes
            pane_grid::Content::new(match &self.panes.get(pane).unwrap() {
                Pane::AppList => container(
                    button(
                        text("Oath")
                            // .align_x(alignment::Horizontal::Center)
                            .width(Fill),
                    )
                    .style(button::secondary)
                    .on_press(Message::OathButtonPress)
                    .width(Fill),
                )
                .style(container::rounded_box)
                .into(),
                Pane::Content => {
                    match &self.content {
                        // Content for Oath app
                        Content::Oath => {
                            let oath_labels = draw_totp_content();
                            oath_labels
                        }
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

fn draw_totp_content() -> iced::Element<'static, Message> {
    let mut devices = solo2::Device::list();
    if devices.len() == 0 {
        // Early return to prevent accessing empty vec
        return text("No Solo2 devices found.").into();
    }
    // Convert from Device type to Solo2 type
    let mut solo2 = devices
        .swap_remove(0)
        .into_solo2()
        .expect("Device is not a solo2 device.");
    // Oath app
    let mut app = Oath::select(&mut solo2).expect("Could not enter oath app.");
    let app_list = app
        .list()
        .unwrap_or_else(|_| vec!["No TOTP codes.".to_string()]);
    // List totp labels and add them to vector of text elements
    let mut oath_labels: Vec<iced::Element<Message>> = vec![];
    // How much time a totp code has left before expiring
    let totp_lifetime = (30
        - (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time before unix epoch somehow.")
            .as_secs()
            % 30)) as f32;

    for label in app_list.iter() {
        let totp = app
            .authenticate(oath::Authenticate::with_label(label))
            .expect("No TOTP with label {label}.");
        oath_labels.push(
            button(
                row![
                    text(label.clone())
                        .width(Fill)
                        .height(Fill)
                        .size(24)
                        .align_y(alignment::Vertical::Center),
                    text(totp)
                        // .height(Fill)
                        .size(32)
                        .align_x(iced::Alignment::End)
                        .align_y(alignment::Vertical::Center),
                    widget::stack![
                        widget::progress_bar(0.00..=30.0, totp_lifetime)
                            .width(40)
                            .height(Fill),
                        text(totp_lifetime)
                            .width(40)
                            .height(Fill)
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center),
                    ]
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
    iced::widget::Column::with_children(oath_labels)
        .spacing(10)
        .width(Fill)
        .into()
}
