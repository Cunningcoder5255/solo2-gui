extern crate iced;
extern crate solo2;
use iced::{
    Element, Fill,
    widget::{
        button, column, container,
        pane_grid::{self, Axis},
        row, text,
    },
};
use solo2::{
    Select, UuidSelectable,
    apps::{Oath, oath},
};

fn main() -> iced::Result {
    // Get devices
    // let state = State::new();

    // Iced
    iced::run("Solo2 Gui", State::update, State::view)
}

enum Pane {
    // Shows solo2 apps like fido and oath
    AppList,
    // Shows the content of these apps, like TOTP keys and fido websites
    Content,
}

enum Content {
    Empty,
    Oath,
}

#[derive(Debug, Clone)]
enum Message {
    OathButtonPress,
    TOTPLabelPress(String /* Label */),
}

struct State {
    panes: pane_grid::State<Pane>,
    content: Content,
}

impl State {
    fn new() -> Self {
        let (mut pane, _) = pane_grid::State::new(Pane::AppList);
        let (first_pane, _) = pane.iter().next().expect("No panes in panegrid.");
        pane.split(Axis::Vertical, *first_pane, Pane::Content);
        let state = State {
            panes: pane,
            content: Content::Empty,
        };
        state
    }
    fn update(state: &mut State, message: Message) -> iced::Task<Message> {
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
        match message {
            Message::OathButtonPress => {
                state.content = Content::Oath;
                iced::Task::none()
            }
            Message::TOTPLabelPress(label) => iced::clipboard::write::<Message>(
                app.authenticate(solo2::apps::oath::Authenticate::with_label(&label))
                    .expect("No TOTP with label: {label}"),
            ),
        }
    }

    // Code concerned with displaying the state and returning messages (showing UI and responding to interaction)
    fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |pane, _, _| {
            // Option<&Pane> will always return because we get pane from &self.panes
            // Add content to AppList and Content panes
            pane_grid::Content::new(match &self.panes.get(pane).unwrap() {
                Pane::AppList => {
                    <iced::widget::Button<'_, Message, iced::Theme, iced::Renderer> as Into<
                        iced::Element<'_, Message, iced::Theme, iced::Renderer>,
                    >>::into(
                        button("Oath").on_press(Message::OathButtonPress).into()
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

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}
