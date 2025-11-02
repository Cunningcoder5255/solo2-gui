use iced::widget::pane_grid::{self, Axis};

pub enum Pane {
    // Shows solo2 apps like fido and oath
    AppList,
    // Shows the content of these apps, like TOTP keys and fido websites
    Content,
}

#[derive(Debug)]
pub enum Content {
    Oath,
}

pub struct State {
    pub panes: pane_grid::State<Pane>,
    pub content: Content,
}

impl State {
    pub fn new() -> Self {
        let (mut pane_grid_state, _) = pane_grid::State::new(Pane::AppList);
        let (first_pane, _) = pane_grid_state
            .iter()
            .next()
            .expect("No panes in panegrid.");
        let (_, split) = pane_grid_state
            .split(Axis::Vertical, *first_pane, Pane::Content)
            .expect("Could not split panegrid.");
        pane_grid_state.resize(split, 0.3);
        let state = State {
            panes: pane_grid_state,
            content: Content::Oath,
        };
        state
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}
