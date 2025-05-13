use iced::widget::{button,column,text};
use iced::{Size, window};

fn main() -> iced::Result {
    iced::application("My App", MyApp::update, MyApp::view)
        .theme(MyApp::theme)
        .window(window::Settings {
            size : Size { width: (405.), height: (720.) },
            ..window::Settings::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message{
    ToggleTheme,
}

#[derive(Default)]
struct MyApp{
    theme: iced::Theme,
}

impl MyApp {
    fn update(&mut self, _message: Message) {
        match _message {
            // Theme swapping, make a cycle around all iced built-in themes 
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    iced::Theme::CatppuccinFrappe => iced::Theme::CatppuccinLatte,
                    iced::Theme::CatppuccinLatte => iced::Theme::CatppuccinMacchiato,
                    iced::Theme::CatppuccinMacchiato => iced::Theme::CatppuccinMocha,
                    iced::Theme::CatppuccinMocha => iced::Theme::Dark,
                    iced::Theme::Dark => iced::Theme::Dracula,
                    iced::Theme::Dracula => iced::Theme::Ferra,
                    iced::Theme::Ferra => iced::Theme::GruvboxDark,
                    iced::Theme::GruvboxDark => iced::Theme::GruvboxLight,
                    iced::Theme::GruvboxLight => iced::Theme::KanagawaDragon,
                    iced::Theme::KanagawaDragon => iced::Theme::KanagawaLotus,
                    iced::Theme::KanagawaLotus => iced::Theme::KanagawaWave,
                    iced::Theme::KanagawaWave => iced::Theme::Light,
                    iced::Theme::Light => iced::Theme::Moonfly,
                    iced::Theme::Moonfly => iced::Theme::Nightfly,
                    iced::Theme::Nightfly => iced::Theme::Nord,
                    iced::Theme::Nord => iced::Theme::Oxocarbon,
                    iced::Theme::Oxocarbon => iced::Theme::SolarizedDark,
                    iced::Theme::SolarizedDark => iced::Theme::SolarizedLight,
                    iced::Theme::SolarizedLight => iced::Theme::TokyoNight,
                    iced::Theme::TokyoNight => iced::Theme::TokyoNightLight,
                    iced::Theme::TokyoNightLight => iced::Theme::TokyoNightStorm,
                    iced::Theme::TokyoNightStorm => iced::Theme::CatppuccinFrappe,
                    _ => iced::Theme::CatppuccinFrappe,
                };
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            button("Changer de thème").on_press(Message::ToggleTheme),
            text("Hello, hackclub!"),
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

}