// GUI crate
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, horizontal_rule, vertical_rule, horizontal_space, vertical_space, row, text},
    window, Length, Size
};

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
            row![
                // Space left to the next field
                horizontal_space().width(Length::FillPortion(1)),
                // Text widget where the user inputs will be displayed
                text("2+2").width(Length::FillPortion(4)).height(80).align_x(Horizontal::Left).align_y(Vertical::Center).size(40),
                vertical_rule(4),
                // Text widget where the result of the calcul  will be displayed
                text("4").width(Length::FillPortion(2)).height(80).align_x(Horizontal::Center).align_y(Vertical::Center).size(40),
            ],
            horizontal_rule(4),
            // Text for Tips
            text("💡 Tips : 2+2 is always equal to 4, you can just memorize it.").shaping(text::Shaping::Advanced).width(Length::Fill).height(80).size(14).align_x(Horizontal::Center).align_y(Vertical::Center),
            horizontal_rule(4),
            vertical_space().height(12),
            // First row : Clear ( ) /
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // Clear button
                button("Ø").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // ( button
                button("(").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // ) button
                button(")").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // / button
                button("÷").padding(30).width(70),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Second row : 7 8 9 *
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // 7 button
                button("7").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 8 button
                button("8").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 9 button
                button("8").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // * button
                button("x").padding(30).width(70),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Third row : 4 5 6 -
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // 4 button
                button("4").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 5 button
                button("5").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 6 button
                button("6").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // - button
                button("-").padding(30).width(70),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Fourth row : 1 2 3 +
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // 1 button
                button("1").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 2 button
                button("2").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 3 button
                button("3").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // + button
                button("+").padding(30).width(70),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // last row : . 0 = Delete
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // . button
                button(".").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 0 button
                button("0").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // = button
                button("=").padding(30).width(70),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // Delete button
                button("←").padding(30).width(70),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Theme swapper button
            row![
                horizontal_space().width(Length::FillPortion(15)),
                button("Theme").on_press(Message::ToggleTheme).padding(20).width(Length::FillPortion(5)),
                horizontal_space().width(Length::FillPortion(2)),
            ],
        ]
        .into()
    }

    fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

}

