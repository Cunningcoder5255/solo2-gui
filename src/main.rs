extern crate iced;
extern crate solo2;
use iced::{
    widget::{container, pane_grid, text},
    Element, Fill,
};
use solo2::{apps::Oath, UuidSelectable};

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

struct Pane {
    id: usize,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

struct State {
    panes: pane_grid::State<Pane>,
}

impl State {
    fn new() -> Self {
        let (pane, _) = pane_grid::State::new(Pane::new(0));
        State { panes: pane }
    }
    fn update(counter: &mut State, message: Message) {
        // match message {
        //    Message::x => ,
        // }
    }

    fn view(&self) -> Element<'_, Message> {
        let pane_grid = iced::widget::PaneGrid::new(&self.panes, |_, _, _| {
            container(text("Text".to_string()))
                .width(Fill)
                .height(Fill)
                .into()
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

#[derive(Debug)]
struct Message;
