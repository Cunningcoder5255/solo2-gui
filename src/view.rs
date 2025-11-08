extern crate cosmic;
use crate::Message;
use crate::state::Pane;
use crate::state::State;
use cosmic::Element;
use cosmic::widget::{self, pane_grid};

pub const SPACING: u16 = 10;

pub fn view<'a>(state: &'a State) -> cosmic::Element<'a, Message> {
    let pane_grid: cosmic::Element<Message> = widget::pane_grid(&state.panes, |pane, _, _| {
        pane_grid::Content::new(match &state.panes.get(pane).unwrap() {
            Pane::AppList => {
                let sidebar: cosmic::Element<Message> =
                    widget::segmented_button::vertical(&state.sidebar)
                        .button_spacing(SPACING)
                        .on_activate(Message::SidebarButtonPress)
                        .into();
                sidebar
            }
            Pane::Content => widget::text("Test.").into(),
        })
    })
    .into();
    pane_grid
}
