// SPDX-License-Identifier: AGPL-3.0

extern crate solo2;
use crate::config::Config;
use crate::fl;
use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::theme;
use cosmic::widget::{self, about::About, icon, menu, nav_bar};
use cosmic::{iced_futures, prelude::*};
use futures_util::SinkExt;
use solo2::apps::{Oath, oath};
use solo2::{Select, UuidSelectable};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/icon.svg");

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
    /// Time active
    time: u32,
    /// Toggle the watch subscription
    watch_is_active: bool,
    /// List of TOTP codes and their respective labels
    totp_list: Vec<(String, String)>,
    /// The Solo2 device we are conected to
    solo2: Option<solo2::Solo2>,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleWatch,
    UpdateConfig(Config),
    WatchTick(u32),
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

        // Set up device and totp_list fields
        let mut solo2_device: Option<solo2::Solo2>;
        let mut devices = solo2::Device::list();
        // Just select first device
        // Maybe add support for multiple devices later
        // I can't test it though because I only have one
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

        // List of totp codes with tuple (Label, TOTP code)
        let mut totp_list: Vec<(String, String)> = vec![];
        if solo2_device.is_some() {
            // Oath app
            // Can unwrap device within is_some()
            let mut app =
                Oath::select(solo2_device.as_mut().unwrap()).expect("Could not enter OATH app:");
            let app_list = app
                .list()
                .unwrap_or_else(|_| vec!["No TOTP codes.".to_string()]);

            for label in app_list.iter() {
                let totp_code = app
                    .authenticate(oath::Authenticate::with_label(&label))
                    .expect("No TOTP");
                totp_list.push((label.to_string(), totp_code));
            }
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
            solo2: solo2_device,
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
            time: 0,
            watch_is_active: false,
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
        const PADDING: u16 = 20;
        // let space_s = cosmic::theme::spacing().space_s;
        let content: Element<_> = match self.nav.active_data::<Page>().unwrap() {
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
                    let totp_code_text = widget::text::title1(totp_code)
                        .width(Length::FillPortion(3))
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .align_x(Alignment::End);
                    let totp_label_text = widget::text::title2(label)
                        .width(Length::FillPortion(1))
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                        .width(Length::Fill);
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
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .into();
                    let totp_container: cosmic::Element<Message> = cosmic::widget::Container::new(
                        widget::row::with_capacity(3)
                            .push(totp_label_text)
                            .push(totp_code_text)
                            .push(totp_lifetime_stack)
                            .spacing(PADDING),
                    )
                    .padding(PADDING)
                    .height(PADDING * 4)
                    .width(Length::Fill)
                    .class(theme::Container::Card)
                    .into();

                    totp_containers.push(totp_container);
                }

                let add_svg =
                    widget::svg::Handle::from_memory(include_bytes!("../svg/add.svg").as_slice());
                let totp_add_button: cosmic::Element<Message> =
                    widget::button::custom(widget::svg(add_svg)).into();
                let divider: cosmic::Element<Message> = widget::divider::horizontal::heavy().into();

                totp_containers.push(divider);
                totp_containers.push(totp_add_button);

                widget::column::with_children(totp_containers)
                    .width(Length::Fill)
                    .into()
            }
        };

        widget::container(content)
            .width(600)
            .height(Length::Fill)
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
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

        // Conditionally enables a timer that emits a message every second.
        if self.watch_is_active {
            subscriptions.push(Subscription::run(|| {
                iced_futures::stream::channel(1, |mut emitter| async move {
                    let mut time = 1;
                    let mut interval = tokio::time::interval(Duration::from_secs(1));

                    loop {
                        interval.tick().await;
                        _ = emitter.send(Message::WatchTick(time)).await;
                        time += 1;
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
        match message {
            Message::WatchTick(time) => {
                self.time = time;
            }

            Message::ToggleWatch => {
                self.watch_is_active = !self.watch_is_active;
            }

            // Message::ToggleContextPage(context_page) => {
            //     if self.context_page == context_page {
            //         // Close the context drawer if the toggled context page is the same.
            //         self.core.window.show_context = !self.core.window.show_context;
            //     } else {
            //         // Open the context drawer to display the requested context page.
            //         self.context_page = context_page;
            //         self.core.window.show_context = true;
            //     }
            // }
            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },
        }
        Task::none()
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Self::Message>> {
        // Activate the page in the model.
        self.nav.activate(id);

        self.update_title()
    }
}

impl AppModel {
    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<cosmic::Action<Message>> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}

/// The page to display in the application.
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
