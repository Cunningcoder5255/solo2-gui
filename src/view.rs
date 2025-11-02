use crate::message::Message;
use crate::state::{Content, Pane, State};
extern crate solo2;
use solo2::apps::{Oath, oath};
use solo2::{Select, UuidSelectable};
extern crate iced;
use iced::{
    Element, Fill, Shrink,
    widget::{button, container, pane_grid, row, text},
};

impl State {
    // Code concerned with displaying the state and returning messages (showing UI and responding to interaction)
    pub fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |pane, _, _| {
            // Option<&Pane> will always return because we get pane from &self.panes
            // Add content to AppList and Content panes
            pane_grid::Content::new(match &self.panes.get(pane).unwrap() {
                Pane::AppList => {
                    <iced::widget::Button<'_, Message, iced::Theme, iced::Renderer> as Into<
                        iced::Element<'_, Message, iced::Theme, iced::Renderer>,
                    >>::into(
                        button("Oath")
                            .on_press(Message::OathButtonPress)
                            .width(Shrink)
                            .height(Shrink)
                            .into(),
                    )
                }
                Pane::Content => {
                    match &self.content {
                        Content::Empty => iced::widget::Container::new(text("")).into(),
                        // Content for Oath app
                        Content::Oath => {
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
                            let mut app =
                                Oath::select(&mut solo2).expect("Could not enter oath app.");
                            let app_list = app
                                .list()
                                .unwrap_or_else(|_| vec!["No TOTP codes.".to_string()]);
                            // List totp labels and add them to vector of text elements
                            let mut oath_labels: Vec<iced::Element<Message>> = vec![];
                            for label in app_list.iter() {
                                let totp = app
                                    .authenticate(oath::Authenticate::with_label(label))
                                    .expect("No TOTP with label {label}.");
                                oath_labels.push(
                                    row![text(label.clone()).width(Fill), text(totp).width(Fill)]
                                        .into(),
                                );
                            }
                            iced::widget::Column::with_children(oath_labels)
                                .width(Fill)
                                .height(Fill)
                                .into()
                        }
                    }
                }
            })
        });
        container(pane_grid)
            .width(Fill)
            .height(Fill)
            .padding(10)
            .into()
    }
}
