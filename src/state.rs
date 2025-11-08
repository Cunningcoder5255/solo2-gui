use crate::view::view;
use cosmic::iced::widget::pane_grid::{self, Axis};
use cosmic::widget;
use solo2::Select;
use solo2::UuidSelectable;
use solo2::apps::{Admin, Oath, oath};

pub enum Pane {
    // Shows solo2 apps like fido and oath
    AppList,
    // Shows the content of these apps, like TOTP keys and fido websites
    Content,
}

#[derive(Debug)]
pub enum Content {
    Oath,
    Admin,
}

pub struct OathState {
    pub adding_totp: bool,
    pub label_input: String,
    pub secret_input: String,
    pub totp_list: Vec<(String, String)>,
    pub deleting_totp: String,          /* label */
    pub invalid_totp_code_length: bool, // When entered TOTP secret is invalid, such as too short, show error to user.
}
impl OathState {
    pub fn new(solo2: &mut Option<solo2::Solo2>) -> Self {
        let mut totp_list: Vec<(String, String)> = vec![];
        if solo2.is_some() {
            totp_list = State::get_device_info(solo2.as_mut().unwrap());
        }
        OathState {
            totp_list: totp_list,
            label_input: "".to_string(),
            secret_input: "".to_string(),
            deleting_totp: "".to_string(),
            invalid_totp_code_length: false,
            adding_totp: false,
        }
    }
}

pub struct AdminState {
    pub locked: bool,
    pub uuid: String,
    pub version: String,
}

impl AdminState {
    pub fn new(solo2: &mut Option<solo2::Solo2>) -> Self {
        let mut uuid = "".to_string();
        let mut version = "".to_string();
        let mut locked = false;

        if solo2.is_some() {
            let mut admin_app =
                Admin::select(solo2.as_mut().unwrap()).expect("Could not enter admin app:");

            locked = admin_app
                .locked()
                .expect("Could not find out if device was locked:");
            uuid = solo2.as_ref().unwrap().uuid().simple().to_string();
            version = solo2.as_ref().unwrap().version().to_semver();
        }

        AdminState {
            locked: locked,
            uuid: uuid,
            version: version,
        }
    }
}

pub struct State {
    pub panes: pane_grid::State<Pane>,
    pub content: Content,
    pub solo2: Option<solo2::Solo2>,
    pub oath_state: OathState,
    pub admin_state: AdminState,
    core: cosmic::core::Core,
    pub sidebar: widget::nav_bar::Model,
}

impl cosmic::Application for State {
    fn init(
        core: cosmic::Core,
        flags: Self::Flags,
    ) -> (Self, cosmic::Task<cosmic::Action<Self::Message>>) {
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

        let mut sidebar_model = widget::nav_bar::Model::default();
        sidebar_model.insert().text("Oath");
        sidebar_model.insert().text("Admin");
        sidebar_model.activate_position(0);

        let state = State {
            panes: pane_grid_state,
            content: Content::Oath,
            oath_state: OathState::new(&mut solo2_device),
            admin_state: AdminState::new(&mut solo2_device),
            solo2: solo2_device,
            core: cosmic::core::Core::default(),
            sidebar: sidebar_model,
        };
        (state, cosmic::Task::none())
    }
    fn view<'a>(&'a self) -> cosmic::iced::Element<'a, Self::Message, cosmic::Theme> {
        view(&self)
    }
    fn core(&self) -> &cosmic::core::Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut cosmic::core::Core {
        &mut self.core
    }
    const APP_ID: &'static str = "com.solo2-gui.solo2-gui";
    type Message = crate::Message;
    type Flags = Vec<(Content, String)>;
    type Executor = cosmic::executor::Default;
}
impl State {
    // Nav bar stuff
    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.sidebar)
    }

    /// Called when a navigation item is selected.
    fn on_nav_select(&mut self, id: widget::nav_bar::Id) {
        self.sidebar.activate(id);
    }

    pub fn update_devices(&mut self) {
        // Get rid of solo2 device to ensure connection to device is broken so it will be reset when the smart card state is refreshed, like when adding or deleting a key
        self.solo2 = Option::None;
        self.solo2 = Self::get_device();
        if self.solo2.is_some() {
            self.oath_state.totp_list = Self::get_device_info(self.solo2.as_mut().unwrap());
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
        let mut app = Oath::select(solo2_device).expect("Could not enter OATH app:");
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

// impl Default for State {
//     fn default() -> Self {
//         State::init()
//     }
// }
