mod search;
mod settings;
mod theme;

use iced::{Element, Fill, Length, Subscription, Task, Theme, time};
use iced::widget::{button, column, container, row, text, text_input};
use iced_webview_v2::{Action as WebAction, PageType, Servo, WebView};
use std::time::Duration;

const HOME: &str = "https://360-search.com";

#[derive(Debug, Clone)]
enum Message {
    AddressChanged(String),
    AddressSubmitted,
    Back,
    Forward,
    Reload,
    OpenSettings,
    CloseSettings,
    SelectEngine(String),
    CustomNameChanged(String),
    CustomTemplateChanged(String),
    AddCustomEngine,
    Animation(settings::AnimationMode),
    Web(WebAction),
    ViewCreated,
    Tick,
}

struct Orbit {
    webview: WebView<Servo, Message>,
    ready: bool,
    show_settings: bool,
    address: String,
    last_page: String,
    search_engine: search::SearchEngine,
    settings: settings::OrbitSettings,
    custom_name: String,
    custom_template: String,
}

impl Orbit {
    fn new() -> (Self, Task<Message>) {
        let settings = settings::OrbitSettings::load();
        let search_engine = settings.search_engine.clone();
        let webview = WebView::new()
            .on_create_view(Message::ViewCreated)
            .on_action(Message::Web)
            .on_url_change(Message::AddressChanged);

        (
            Self {
                webview,
                ready: false,
                show_settings: false,
                address: HOME.into(),
                last_page: HOME.into(),
                search_engine,
                settings,
                custom_name: String::new(),
                custom_template: String::new(),
            },
            Task::done(Message::Web(WebAction::CreateView(PageType::Url(HOME.into())))),
        )
    }

    fn navigate(&mut self) -> Task<Message> {
        let input = self.address.trim();
        if input == "orbit://settings" {
            self.show_settings = true;
            return Task::none();
        }
        if input.is_empty() { return Task::none(); }
        let target = search::resolve_input(input, &self.search_engine);
        self.last_page = target.clone();
        self.show_settings = false;
        self.webview.update(WebAction::Navigate(target))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddressChanged(url) => {
                if !self.show_settings { self.address = url.clone(); self.last_page = url; }
                Task::none()
            }
            Message::AddressSubmitted => self.navigate(),
            Message::Back => self.webview.update(WebAction::GoBack),
            Message::Forward => self.webview.update(WebAction::GoForward),
            Message::Reload => self.webview.update(WebAction::Reload),
            Message::OpenSettings => {
                self.show_settings = true;
                self.address = "orbit://settings".into();
                Task::none()
            }
            Message::CloseSettings => {
                self.show_settings = false;
                self.address = self.last_page.clone();
                Task::none()
            }
            Message::SelectEngine(name) => {
                if let Some(engine) = self.settings.engines.iter().find(|e| e.name == name).cloned() {
                    self.search_engine = engine.clone();
                    self.settings.search_engine = engine;
                    self.settings.save();
                }
                Task::none()
            }
            Message::CustomNameChanged(value) => { self.custom_name = value; Task::none() }
            Message::CustomTemplateChanged(value) => { self.custom_template = value; Task::none() }
            Message::AddCustomEngine => {
                let name = self.custom_name.trim();
                let template = self.custom_template.trim();
                if !name.is_empty() && template.contains("%s") {
                    let engine = search::SearchEngine { name: name.into(), template: template.into() };
                    self.settings.engines.push(engine.clone());
                    self.settings.search_engine = engine.clone();
                    self.search_engine = engine;
                    self.settings.save();
                    self.custom_name.clear();
                    self.custom_template.clear();
                }
                Task::none()
            }
            Message::Animation(mode) => {
                self.settings.animations = mode;
                self.settings.save();
                Task::none()
            }
            Message::Web(action) => self.webview.update(action),
            Message::ViewCreated => {
                self.ready = true;
                self.webview.update(WebAction::ChangeView(0))
            }
            Message::Tick => self.webview.update(WebAction::Update),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let nav = row![
            chrome_button("‹", Message::Back),
            chrome_button("›", Message::Forward),
            chrome_button("↻", Message::Reload),
            text_input("Search 360 or enter an address", &self.address)
                .on_input(Message::AddressChanged)
                .on_submit(Message::AddressSubmitted)
                .padding(11)
                .width(Length::Fill)
                .size(14),
            chrome_button("Settings", Message::OpenSettings),
        ]
        .spacing(8)
        .padding(10)
        .align_y(iced::Center);

        let content: Element<'_, Message> = if self.show_settings {
            self.settings_view()
        } else if self.ready {
            self.webview.view().map(Message::Web)
        } else {
            container(text("Starting Orbit…").size(16)).center(Fill).into()
        };

        column![nav, content]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn settings_view(&self) -> Element<'_, Message> {
        let engine_buttons = self.settings.engines.iter().map(|engine| {
            let active = engine.name == self.search_engine.name;
            button(text(if active { format!("✓  {}", engine.name) } else { engine.name.clone() }))
                .on_press(Message::SelectEngine(engine.name.clone()))
                .padding([10, 14])
                .style(theme::ripple_button)
                .into()
        }).collect::<Vec<Element<'_, Message>>>();

        let animation_buttons = vec![
            animation_button("Full", settings::AnimationMode::Full, self.settings.animations),
            animation_button("Reduced", settings::AnimationMode::Reduced, self.settings.animations),
            animation_button("Off", settings::AnimationMode::Off, self.settings.animations),
        ];

        container(column![
            row![text("Orbit Settings").size(30), button(text("Back to page")).on_press(Message::CloseSettings).style(theme::ripple_button)]
                .spacing(12).align_y(iced::Center),
            text("Browser preferences are stored locally. No database is required.").size(14),
            section("Animation control", row![animation_buttons[0].clone(), animation_buttons[1].clone(), animation_buttons[2].clone()].spacing(8)),
            section("Default search engine", column![row(engine_buttons).spacing(8), text(format!("Current: {}", self.search_engine.name)).size(13)].spacing(8)),
            section("Add custom search engine", column![
                text_input("Engine name", &self.custom_name).on_input(Message::CustomNameChanged).padding(10),
                text_input("URL template — include %s", &self.custom_template).on_input(Message::CustomTemplateChanged).padding(10),
                button(text("Add engine")).on_press(Message::AddCustomEngine).padding([10, 14]).style(theme::ripple_button),
            ].spacing(8)),
            section("Orbit", column![
                text("Home: https://360-search.com"),
                text("Search: https://360-search.com/search?q-%s"),
                text("Engine: Servo"),
                text("No Chromium / Electron / WebView2 / Google dependency"),
            ].spacing(5)),
        ].spacing(18).padding(32)).width(Length::Fill).height(Length::Fill).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }
}

fn section<'a>(title: &'a str, body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(column![text(title).size(18), body.into()].spacing(10).padding(18))
        .width(Length::Fill)
        .style(theme::section)
        .into()
}

fn animation_button(label: &'static str, mode: settings::AnimationMode, active: settings::AnimationMode) -> Element<'static, Message> {
    button(text(if mode == active { format!("✓  {label}") } else { label }))
        .on_press(Message::Animation(mode))
        .padding([10, 14])
        .style(theme::ripple_button)
        .into()
}

fn chrome_button<'a>(label: &'a str, msg: Message) -> Element<'a, Message> {
    button(text(label).size(14).center())
        .on_press(msg)
        .padding([9, 14])
        .style(theme::ripple_button)
        .into()
}

fn main() -> iced::Result {
    iced::application(Orbit::new, Orbit::update, Orbit::view)
        .title("Orbit Browser")
        .theme(|_| Theme::Dark)
        .subscription(Orbit::subscription)
        .run()
}
