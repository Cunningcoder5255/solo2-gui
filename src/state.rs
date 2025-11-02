use iced::widget::pane_grid::{self, Axis};

pub enum Pane {
    // Shows solo2 apps like fido and oath
    AppList,
    // Shows the content of these apps, like TOTP keys and fido websites
    Content,
}

#[derive(Debug)]
pub enum Content {
    Empty,
    Oath,
}

pub struct State {
    pub panes: pane_grid::State<Pane>,
    pub content: Content,
}

impl State {
    pub fn new() -> Self {
        let (mut pane, _) = pane_grid::State::new(Pane::AppList);
        let (first_pane, _) = pane.iter().next().expect("No panes in panegrid.");
        pane.split(Axis::Vertical, *first_pane, Pane::Content);
        let state = State {
            panes: pane,
            content: Content::Empty,
        };
        state
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}
