use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, vertical_space, Container},
    Element, Theme,
};
use iced_aw::tab_bar::TabLabel;
use crate::{Icon, Message, Tab};


#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeSwitched,
}


#[derive(Default)]
pub struct SettingsTab {
    pub theme: Theme,
}

impl SettingsTab {

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::ThemeSwitched => self.theme = match self.theme {
                Theme::CatppuccinFrappe => Theme::CatppuccinLatte,
                Theme::CatppuccinLatte => Theme::CatppuccinMacchiato,
                Theme::CatppuccinMacchiato => Theme::CatppuccinMocha,
                Theme::CatppuccinMocha => Theme::Dark,
                Theme::Dark => Theme::Dracula,
                Theme::Dracula => Theme::Ferra,
                Theme::Ferra => Theme::GruvboxDark,
                Theme::GruvboxDark => Theme::GruvboxLight,
                Theme::GruvboxLight => Theme::KanagawaDragon,
                Theme::KanagawaDragon => Theme::KanagawaLotus,
                Theme::KanagawaLotus => Theme::KanagawaWave,
                Theme::KanagawaWave => Theme::Light,
                Theme::Light => Theme::Moonfly,
                Theme::Moonfly => Theme::Nightfly,
                Theme::Nightfly => Theme::Nord,
                Theme::Nord => Theme::Oxocarbon,
                Theme::Oxocarbon => Theme::SolarizedDark,
                Theme::SolarizedDark => Theme::SolarizedLight,
                Theme::SolarizedLight => Theme::TokyoNight,
                Theme::TokyoNight => Theme::TokyoNightLight,
                Theme::TokyoNightLight => Theme::TokyoNightStorm,
                Theme::TokyoNightStorm => Theme::CatppuccinFrappe,
                _ => Theme::CatppuccinFrappe,
            }
        }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Settings.into(), String::from("Settings"))
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, SettingsMessage> = Container::new(
            column![
                vertical_space().height(12),
                button("Thème").padding(30).on_press(SettingsMessage::ThemeSwitched),
            ]
        )
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();

        content.map(Message::Settings)
    }
}