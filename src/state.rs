use iced::widget::pane_grid::{self, Axis};
use solo2::Select;
use solo2::UuidSelectable;
use solo2::apps::{Oath, oath};

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
    pub adding_totp: bool,
    pub label_input: String,
    pub secret_input: String,
    pub solo2: Option<solo2::Solo2>,
    pub totp_list: Vec<(String, String)>,
}

impl State {
    pub fn new() -> Self {
        // Set up pane split
        let (mut pane_grid_state, _) = pane_grid::State::new(Pane::AppList);
        let (first_pane, _) = pane_grid_state
            .iter()
            .next()
            .expect("No panes in panegrid.");
        let (_, split) = pane_grid_state
            .split(Axis::Vertical, *first_pane, Pane::Content)
            .expect("Could not split panegrid.");
        pane_grid_state.resize(split, 0.3);

        let mut solo2_device = State::get_device();
        let mut totp_list: Vec<(String, String)> = vec![];

        if solo2_device.is_some() {
            totp_list = State::get_device_info(solo2_device.as_mut().unwrap());
        }

        let state = State {
            panes: pane_grid_state,
            content: Content::Oath,
            adding_totp: false,
            label_input: "".to_string(),
            secret_input: "".to_string(),
            solo2: solo2_device,
            totp_list: totp_list,
        };
        state
    }
    pub fn update_devices(&mut self) {
        self.solo2 = Self::get_device();
        if self.solo2.is_some() {
            self.totp_list = Self::get_device_info(self.solo2.as_mut().unwrap());
        }
    }
    fn get_device() -> Option<solo2::Solo2> {
        // Set up device and totp_list fields
        let solo2_device: Option<solo2::Solo2>;
        let mut devices = solo2::Device::list();
        if devices.len() == 0 {
            solo2_device = Option::None;
        } else {
            // Convert from Device type to Solo2 type
            solo2_device = Option::Some(
                devices
                    .swap_remove(0)
                    .into_solo2()
                    .expect("Device is not a solo2 device."),
            );
        }
        solo2_device
    }
    fn get_device_info(solo2_device: &mut solo2::Solo2) -> Vec<(String, String)> {
        // Oath app
        let mut app = Oath::select(solo2_device).expect("Could not enter oath app.");
        let app_list = app
            .list()
            .unwrap_or_else(|_| vec!["No TOTP codes.".to_string()]);
        let mut totp_list: Vec<(String, String)> = vec![];

        for label in app_list.iter() {
            let totp_code = app
                .authenticate(oath::Authenticate::with_label(&label))
                .expect("No TOTP");
            totp_list.push((label.to_string(), totp_code));
        }
        totp_list
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}
