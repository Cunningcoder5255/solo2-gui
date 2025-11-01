extern crate iced;
extern crate solo2;
use iced::{
    widget::{
        container,
        pane_grid::{self, Axis},
        text,
        button,
        span,
    },
    Element, Fill,
};
// use solo2::{apps::Oath, UuidSelectable};

fn main() -> iced::Result {
    // Get devices
    // let mut devices = solo2::Device::list();
    // Convert from Device type to Solo2 type
    //let mut solo2 = devices.swap_remove(0).into_solo2().expect("Device is not a solo2 device.");
    // let mut app = Oath::select(&mut solo2).expect("Could not enter oath app.");
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

#[derive(Debug,Clone)]
enum Message {
    OathButtonPress,
}

struct State {
    panes: pane_grid::State<Pane>,
}

impl State {
    fn new() -> Self {
        let (mut pane, _) = pane_grid::State::new(Pane::AppList);
        let (first_pane, _) = pane.iter().next().expect("No panes in panegrid.");
        pane.split(Axis::Vertical, *first_pane, Pane::Content);
        let state = State { panes: pane, };
        state
    }
    fn update(counter: &mut State, message: Message) {
        match message {
           Message::OathButtonPress => println!("Button Pressed"),
        }
    }

    // Code concerned with displaying the state and returning messages (showing UI and responding to interaction)
    fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |pane, _, _| {
            // Option<&Pane> will always return because we get pane from &self.panes
            // Add content to AppList and Content panes
            pane_grid::Content::new(
            match &self.panes.get(pane).unwrap() {
                Pane::AppList => {
                    <iced::widget::Button<'_, Message, iced::Theme, iced::Renderer> as Into<iced::Element<'_, Message, iced::Theme, iced::Renderer>>>::into(button("Oath").on_press(Message::OathButtonPress).into())
                },
                Pane::Content => {
                    text("Test".to_string()).into()
                },
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
