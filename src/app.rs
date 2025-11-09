// SPDX-License-Identifier: AGPL-3.0

extern crate solo2;
use crate::config::Config;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::theme;
use cosmic::widget::{self, icon, nav_bar};
use cosmic::{iced_futures, prelude::*};
use futures_util::SinkExt;
use solo2::apps::{Oath, oath};
use solo2::{Select, UuidSelectable};
use std::time::Duration;
use std::time::SystemTime;

// const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
// const APP_ICON: &[u8] = include_bytes!("../svg/copy.svg"); // TODO: Add icon

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// Display a context drawer with the designated page if defined.
    // context_page: ContextPage,
    /// The about page for this app.
    // about: About,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    // key_binds: HashMap<menu::KeyBind, MenuAction>,
    /// Configuration data that persists between application runs.
    config: Config,
    /// List of TOTP codes and their respective labels
    totp_list: Vec<(String, String)>,
    /// The Solo2 device we are conected to
    solo2: Option<solo2::Solo2>,
    /// Whether to show the widget for adding a totp code or the add button
    adding_totp: bool,
    /// The current content of the label input for the add totp widget
    label_input: String,
    /// The current content of the secret for the add totp widget
    secret_input: String,
    /// If the input secret was not 16 characters (to present an error message)
    invalid_totp_code_length: bool,
    /// The TOTP we asking to confirm deletion of, "" if none
    deleting_totp: Option<String>,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    // Cancel deleting a TOTP code
    CancelDeleteTOTP,
    // Delete totp code with specified label
    DeleteTOTP(String),
    // Prompt if the user is really sure they want to delete the TOTP code with label String
    PromptDeleteTOTP(String),
    // Update TOTP Lifespan display every second
    RefreshTOTPLifespan,
    // Copy a TOTP code to clipboard
    CopyTOTP(String),
    AddTOTPButton,
    CancelAddTOTP,
    AddTOTPCode,
    UpdateLabelInput(String),
    UpdateSecretInput(String),
    UpdateConfig(Config),
}

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.github.cunningcoder5255.solo2-gui";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Create a nav bar with three page items.
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text("Oath")
            .data::<Page>(Page::Oath)
            .icon(icon::from_name("applications-science-symbolic"))
            .activate();

        nav.insert()
            .text("Admin")
            .data::<Page>(Page::Admin)
            .icon(icon::from_name("applications-system-symbolic"));
        let mut solo2 = AppModel::get_device();
        let mut totp_list: Vec<(String, String)> = vec![];
        if solo2.is_some() {
            totp_list = AppModel::get_device_info(solo2.as_mut().unwrap());
        }

        // Create the about widget
        // let about = About::default()
        //     .name(fl!("app-title"))
        //     .icon(widget::icon::from_svg_bytes(APP_ICON))
        //     .version(env!("CARGO_PKG_VERSION"))
        //     .links([(fl!("repository"), REPOSITORY)])
        //     .license(env!("CARGO_PKG_LICENSE"));

        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            // context_page: ContextPage::default(),
            // about,
            totp_list,
            solo2,
            adding_totp: false,
            label_input: "".to_string(),
            secret_input: "".to_string(),
            deleting_totp: None,
            invalid_totp_code_length: false,
            nav,
            // key_binds: HashMap::new(),
            // Optional configuration file for an application.
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => {
                        // for why in errors {
                        //     tracing::error!(%why, "error loading app config");
                        // }

                        config
                    }
                })
                .unwrap_or_default(),
        };

        // Create a startup command that sets the window title.
        let command = app.update_title();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    // fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
    //     let menu_bar = menu::bar(vec![menu::Tree::with_children(
    //         menu::root(fl!("view")).apply(Element::from),
    //         menu::items(
    //             &self.key_binds,
    //             vec![menu::Item::Button(fl!("about"), None)],
    //         ),
    //     )]);

    //     vec![menu_bar.into()]
    // }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    // /// Display a context drawer if the context page is requested.
    // fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<'_, Self::Message>> {
    //     if !self.core.window.show_context {
    //         return None;
    //     }

    //     Some(match self.context_page {
    //         ContextPage::About => context_drawer::about(
    //             &self.about,
    //             |url| Message::LaunchUrl(url.to_string()),
    //             Message::ToggleContextPage(ContextPage::About),
    //         ),
    //     })
    // }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<'_, Self::Message> {
        let padding: u16 = cosmic::theme::spacing().space_xs;
        let xxxl_spacing: u16 = cosmic::theme::spacing().space_xxxl;
        // let space_s = cosmic::theme::spacing().space_s;
        match self.nav.active_data::<Page>().unwrap() {
            Page::Admin => widget::text("Admin page").into(),

            Page::Oath => {
                // If there aren't any solo2 devices, tell the user and return early since there won't be any codes
                if !&self.solo2.is_some() {
                    return widget::text("No solo2 devices.").into();
                }
                // TODO: Add message when there are no totp codes
                let mut totp_containers: Vec<cosmic::Element<Message>> = vec![];
                let totp_list = &self.totp_list;

                // How much time a totp code has left before expiring
                let totp_lifetime = (30
                    - (SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("System time before unix epoch somehow.")
                        .as_secs()
                        % 30)) as f32;

                // Loop over the totp info and add the label and code to a card and add the card to the totp_containers collection
                for (label, totp_code) in totp_list.into_iter() {
                    let delete_svg = widget::svg::Handle::from_memory(
                        include_bytes!("../svg/trash.svg").as_slice(),
                    );
                    let totp_delete_button: cosmic::Element<Message> = widget::container(
                        widget::button::custom(
                            widget::container(widget::svg(delete_svg).width(Length::Shrink))
                                .center_y(Length::Shrink)
                                .width(Length::Shrink),
                        )
                        .on_press(Message::PromptDeleteTOTP(label.clone()))
                        .class(cosmic::theme::Button::Destructive)
                        .width(Length::Shrink)
                        .height(Length::Fill),
                    )
                    .center_y(Length::Shrink)
                    .width(Length::Shrink)
                    .into();
                    let totp_label_text = widget::text::title2(label)
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .width(Length::Shrink);
                    let copy_svg = widget::svg::Handle::from_memory(
                        include_bytes!("../svg/copy.svg").as_slice(),
                    );
                    let copy_totp_button: cosmic::Element<Message> =
                        widget::button::custom(widget::svg(copy_svg).width(Length::Shrink))
                            .width(Length::Shrink)
                            .height(Length::Shrink)
                            .on_press(Message::CopyTOTP(label.clone()))
                            .into();
                    let totp_code_text = widget::text::title1(totp_code)
                        .width(Length::Shrink)
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .align_x(Alignment::End);
                    let totp_lifetime_stack: cosmic::Element<Message> = widget::container(
                        cosmic::iced::widget::stack!(
                            cosmic::widget::progress_bar(-5.0..=30.0, totp_lifetime)
                                .height(Length::Fill),
                            widget::text::title3(totp_lifetime.to_string())
                                .center()
                                .width(Length::Fill)
                                .height(Length::Fill)
                        )
                        .height(Length::Fill),
                    )
                    .width(40)
                    .height(Length::Fill)
                    .into();
                    let totp_container: cosmic::Element<Message> = cosmic::widget::Container::new(
                        widget::row::with_capacity(2)
                            .push(widget::container(
                                widget::row::with_capacity(2)
                                    .push(totp_delete_button)
                                    .push(totp_label_text)
                                    .spacing(padding),
                            ))
                            .push(
                                widget::container(
                                    widget::row::with_capacity(3)
                                        .push(copy_totp_button)
                                        .push(totp_code_text)
                                        .push(totp_lifetime_stack)
                                        .spacing(padding),
                                )
                                .align_right(Length::Fill),
                            )
                            .spacing(padding),
                    )
                    .padding(padding)
                    .height(60)
                    .width(Length::Fill)
                    .class(theme::Container::Card)
                    .into();

                    totp_containers.push(totp_container);
                }

                let divider: cosmic::Element<Message> = widget::row::with_capacity(3)
                    .push(widget::Space::with_width(xxxl_spacing))
                    .push(widget::divider::horizontal::default().width(Length::Fill))
                    .push(widget::Space::with_width(xxxl_spacing))
                    .into();
                totp_containers.push(divider);

                if self.adding_totp {
                    let mut invalid_totp_dialog = widget::text("");
                    if self.invalid_totp_code_length {
                        invalid_totp_dialog =
                            widget::text("Secret length should be 16 characters.");
                    }
                    let label_input = widget::text_input("Label", self.label_input.clone())
                        .on_input(Message::UpdateLabelInput);
                    let secret_input = widget::text_input("Secret", self.secret_input.clone())
                        .on_input(Message::UpdateSecretInput);
                    let add_button = widget::button::text("Add")
                        .on_press(Message::AddTOTPCode)
                        .class(cosmic::theme::Button::Suggested);
                    let cancel_button =
                        widget::button::text("Cancel").on_press(Message::CancelAddTOTP);
                    let adding_totp_widget: cosmic::Element<Message> =
                        widget::column::with_capacity(2)
                            .push(
                                widget::container(
                                    widget::row::with_capacity(2)
                                        .push(label_input)
                                        .push(secret_input)
                                        .spacing(padding),
                                )
                                .class(cosmic::theme::Container::Card)
                                .padding(padding),
                            )
                            .push(
                                widget::container(
                                    widget::row::with_capacity(3)
                                        .push(invalid_totp_dialog)
                                        .push(cancel_button)
                                        .push(add_button)
                                        .spacing(padding),
                                )
                                .width(Length::Fill)
                                .align_x(Alignment::End),
                            )
                            .spacing(padding)
                            .into();
                    totp_containers.push(adding_totp_widget);
                } else {
                    let add_svg = widget::svg::Handle::from_memory(
                        include_bytes!("../svg/add.svg").as_slice(),
                    );
                    let totp_add_button: cosmic::Element<Message> =
                        widget::button::custom(widget::svg(add_svg))
                            .on_press(Message::AddTOTPButton)
                            .into();
                    totp_containers.push(totp_add_button);
                }
                let dialog: cosmic::Element<Message>;

                if self.deleting_totp.is_some() {
                    let confirmation_text = "Are you sure you want to delete TOTP code \""
                        .to_string()
                        + &self.deleting_totp.clone().unwrap()
                        + "\"?";
                    let cancel_button: cosmic::Element<Message> = widget::button::text("Cancel")
                        .on_press(Message::CancelDeleteTOTP)
                        .into();
                    let delete_button: cosmic::Element<Message> =
                        widget::button::destructive("Delete")
                            .on_press(Message::DeleteTOTP(self.deleting_totp.clone().unwrap()))
                            .into();

                    dialog = cosmic::widget::dialog()
                        .title("Confirm Deletion")
                        .body(confirmation_text)
                        .primary_action(cancel_button)
                        .secondary_action(delete_button)
                        .into();
                } else {
                    dialog = widget::text("").into()
                }

                widget::container(cosmic::iced::widget::stack![
                    widget::column::with_children(totp_containers)
                        .spacing(padding)
                        .width(Length::Fill),
                    widget::container(dialog)
                        .height(Length::Fill)
                        .center(Length::Fill)
                ])
                .width(1000)
                .height(Length::Fill)
                .apply(widget::container)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
            }
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They can be dynamically
    /// stopped and started conditionally based on application state, or persist
    /// indefinitely.
    fn subscription(&self) -> Subscription<Self::Message> {
        // Add subscriptions which are always active.
        let mut subscriptions = vec![
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ];

        let active_page: &Page = self.nav.active_data().unwrap();

        // Conditionally enables a timer that emits a message every second.
        if *active_page == Page::Oath {
            subscriptions.push(Subscription::run(|| {
                iced_futures::stream::channel(1, |mut emitter| async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(1));

                    loop {
                        interval.tick().await;
                        _ = emitter.send(Message::RefreshTOTPLifespan).await;
                    }
                })
            }));
        }

        Subscription::batch(subscriptions)
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        let mut task: Option<cosmic::Task<cosmic::Action<Message>>> = None;
        match message {
            Message::CopyTOTP(label) => {
                let solo2 = self.solo2.as_mut().unwrap();
                let mut app = Oath::select(solo2).expect("Could not enter oath app.");
                task = Some(cosmic::iced::clipboard::write::<cosmic::Action<Message>>(
                    app.authenticate(solo2::apps::oath::Authenticate::with_label(&label))
                        .expect("No TOTP with label: {label}"),
                ));
            }
            Message::RefreshTOTPLifespan => (),
            Message::PromptDeleteTOTP(label) => self.deleting_totp = Some(label),
            Message::CancelDeleteTOTP => self.deleting_totp = None,
            Message::DeleteTOTP(label) => {
                let solo2 = self.solo2.as_mut().unwrap();
                let mut app = Oath::select(solo2).expect("Could not enter oath app.");
                app.delete(label).expect("Could not delete TOTP.");
                // Update TOTP list to reflect the deleted entry
                self.update_devices();
                // No longer prompting to delete TOTP code
                self.deleting_totp = None;
            }
            Message::UpdateLabelInput(label) => self.label_input = label,
            Message::UpdateSecretInput(secret) => self.secret_input = secret,
            Message::CancelAddTOTP => {
                self.adding_totp = false;
            }
            Message::AddTOTPCode => {
                if self.secret_input.len() != 16 {
                    self.invalid_totp_code_length = true;
                } else {
                    self.invalid_totp_code_length = false;
                    let solo2 = self.solo2.as_mut().unwrap(); // Can unwrap because totp screen won't be shown if there are no devices
                    let mut app = Oath::select(solo2).expect("Could not enter oath app.");

                    app.register(
                        solo2::apps::oath::Credential::default_totp(
                            &self.label_input,
                            &self.secret_input,
                        )
                        .expect("Could not get credential"),
                    )
                    .expect("Could not register TOTP code.");
                    // Clear inputs and get out of adding_totp screen
                    self.adding_totp = false;
                    self.update_devices();
                }
            }
            Message::AddTOTPButton => {
                // Get clean input state every time
                self.secret_input = "".to_string();
                self.label_input = "".to_string();
                self.adding_totp = true;
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }
        }
        if task.is_some() {
            return task.unwrap();
        }
        Task::none()
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Self::Message>> {
        // Activate the page in the model.
        self.nav.activate(id);

        cosmic::Task::none()
    }
}

impl AppModel {
    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<cosmic::Action<Message>> {
        let window_title = "Solo 2 GUI".to_string();

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
    pub fn update_devices(&mut self) {
        // Get rid of solo2 device to ensure connection to device is broken so it will be reset when the smart card state is refreshed, like when adding or deleting a key
        self.solo2 = Option::None;
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

/// The page to display in the application.
#[derive(Eq, PartialEq)]
pub enum Page {
    Oath,
    Admin,
}

// /// The context page to display in the context drawer.
// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
// pub enum ContextPage {
//     #[default]
//     About,
// }

// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum MenuAction {
//     About,
// }

// impl menu::action::MenuAction for MenuAction {
//     type Message = Message;

//     fn message(&self) -> Self::Message {
//         match self {
//             MenuAction::About => Message::ToggleContextPage(ContextPage::About),
//         }
//     }
// }
