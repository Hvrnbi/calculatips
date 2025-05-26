// GUI crate
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, horizontal_rule, vertical_rule, horizontal_space, vertical_space, row, text},
    window, Length, Size
};
use evalexpr::{eval};

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
    // Theme button message
    ToggleTheme,
    // Numbers messages
    Pressed0,
    Pressed1,
    Pressed2,
    Pressed3,
    Pressed4,
    Pressed5,
    Pressed6,
    Pressed7,
    Pressed8,
    Pressed9,
    // Other messages
    PressedDot,
    PressedEqual,
    PressedDel,
    PressedClear,
    PressedOpeningParanthesis,
    PressedClosingParanthesis,
    PressedAddition,
    PressedSubstraction,
    PressedMultiplication,
    PressedDivision,
}

#[derive(Default)]
struct MyApp{
    theme: iced::Theme,
    displayed_calc: String,
    calc: String,
    result: String,
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
            },
            // When then keyboard buttons are pressed
            Message::Pressed0 => if self.displayed_calc.len()<30 {self.displayed_calc += "0"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "0"} else {self.calc += "0."}},
            Message::Pressed1 => if self.displayed_calc.len()<30 {self.displayed_calc += "1"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "1"} else {self.calc += "1."}},
            Message::Pressed2 => if self.displayed_calc.len()<30 {self.displayed_calc += "2"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "2"} else {self.calc += "2."}},
            Message::Pressed3 => if self.displayed_calc.len()<30 {self.displayed_calc += "3"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "3"} else {self.calc += "3."}},
            Message::Pressed4 => if self.displayed_calc.len()<30 {self.displayed_calc += "4"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "4"} else {self.calc += "4."}},
            Message::Pressed5 => if self.displayed_calc.len()<30 {self.displayed_calc += "5"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "5"} else {self.calc += "5."}},
            Message::Pressed6 => if self.displayed_calc.len()<30 {self.displayed_calc += "6"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "6"} else {self.calc += "6."}},
            Message::Pressed7 => if self.displayed_calc.len()<30 {self.displayed_calc += "7"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "7"} else {self.calc += "7."}},
            Message::Pressed8 => if self.displayed_calc.len()<30 {self.displayed_calc += "8"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "8"} else {self.calc += "8."}},
            Message::Pressed9 => if self.displayed_calc.len()<30 {self.displayed_calc += "9"; if in_float(&self.calc){if &self.displayed_calc[self.displayed_calc.len()-2..self.displayed_calc.len()-1] != "." {self.calc = self.calc[..self.calc.len()-1].to_string()} self.calc += "9"} else {self.calc += "9."}},
            Message::PressedAddition => if self.displayed_calc.len()<30 {self.displayed_calc += "+"; self.calc += "+"},
            Message::PressedSubstraction => if self.displayed_calc.len()<30 {self.displayed_calc += "-"; self.calc += "-"},
            Message::PressedMultiplication => if self.displayed_calc.len()<30 {self.displayed_calc += "x"; self.calc += "*"},
            Message::PressedDivision => if self.displayed_calc.len()<30 {self.displayed_calc += "÷"; self.calc += "/"},
            Message::PressedOpeningParanthesis => if self.displayed_calc.len()<30 {self.displayed_calc += "("; self.calc += "("},
            Message::PressedClosingParanthesis => if self.displayed_calc.len()<30 {self.displayed_calc += ")"; self.calc += ")"},
            Message::PressedDot => if self.displayed_calc.len()<30 && !in_float(&self.displayed_calc) {self.displayed_calc += "."; if &self.calc[self.calc.len()-1..] != "." {self.calc+= "."} } ,
            Message::PressedDel => if self.displayed_calc!="" {self.displayed_calc = self.displayed_calc[..self.displayed_calc.len()-1].to_string(); if &self.calc[self.calc.len()-1..] == "." {self.calc=self.calc[..self.calc.len()-2].to_string()} else {self.calc=self.calc[..self.calc.len()-1].to_string()}},
            Message::PressedClear => {self.displayed_calc.clear(); self.calc.clear(); self.result.clear();},
            Message::PressedEqual => self.result = eval(&self.calc).expect("None").to_string(),
        }
    }
 
    fn view(&self) -> iced::Element<Message> {
        column![
            row![
                // Space left to the next field
                horizontal_space().width(Length::FillPortion(1)),
                // Text widget where the user inputs will be displayed
                text(self.displayed_calc.clone()).width(Length::FillPortion(6)).height(80).align_x(Horizontal::Left).align_y(Vertical::Center).size(32),
                vertical_rule(4),
                // Text widget where the result of the calcul  will be displayed
                text(self.result.to_string()).width(Length::FillPortion(2)).height(80).align_x(Horizontal::Center).align_y(Vertical::Center).size(32),
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
                button("Ø").padding(30).width(70).on_press(Message::PressedClear),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // ( button
                button("(").padding(30).width(70).on_press(Message::PressedOpeningParanthesis),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // ) button
                button(")").padding(30).width(70).on_press(Message::PressedClosingParanthesis),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // / button
                button("÷").padding(30).width(70).on_press(Message::PressedDivision),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Second row : 7 8 9 *
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // 7 button
                button("7").padding(30).width(70).on_press(Message::Pressed7),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 8 button
                button("8").padding(30).width(70).on_press(Message::Pressed8),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 9 button
                button("9").padding(30).width(70).on_press(Message::Pressed9),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // * button
                button("x").padding(30).width(70).on_press(Message::PressedMultiplication),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Third row : 4 5 6 -
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // 4 button
                button("4").padding(30).width(70).on_press(Message::Pressed4),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 5 button
                button("5").padding(30).width(70).on_press(Message::Pressed5),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 6 button
                button("6").padding(30).width(70).on_press(Message::Pressed6),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // - button
                button("-").padding(30).width(70).on_press(Message::PressedSubstraction),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // Fourth row : 1 2 3 +
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // 1 button
                button("1").padding(30).width(70).on_press(Message::Pressed1),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 2 button
                button("2").padding(30).width(70).on_press(Message::Pressed2),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 3 button
                button("3").padding(30).width(70).on_press(Message::Pressed3),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // + button
                button("+").padding(30).width(70).on_press(Message::PressedAddition),
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
            ],
            vertical_space().height(12),
            // last row : . 0 = Delete
            row![
                // Space left to the first button
                horizontal_space().width(Length::FillPortion(2)),
                // . button
                button(".").padding(30).width(70).on_press(Message::PressedDot),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // 0 button
                button("0").padding(30).width(70).on_press(Message::Pressed0),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // = button
                button("=").padding(30).width(70).on_press(Message::PressedEqual),
                // Space between two buttons
                horizontal_space().width(Length::FillPortion(1)),
                // Delete button
                button("←").padding(30).width(70).on_press(Message::PressedDel),
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


fn in_float(calc: &String) -> bool {
    if calc == "" {return false}
    for (_i, &item) in calc.as_bytes().iter().enumerate().rev() {
        if item == b'.' {return true}
        else if item == b'+' || item == b'-' || item == b'*' || item == b'/' /*|| item == b'x' || item == b'\xF7'*/ {return false}
    }
    false
}




// It's hard so I use a crate, I'll make it myself later if I have the time

// A simplified Python eval() function
//fn eval(calc:&String) -> f64 {
//    for (_i,car) in calc.chars().enumerate() {
//        if car=='(' {
//            // I'll make it later
//        } else if car=='0' || car=='1' || car=='2' || car=='3' || car=='4' || car=='5' || car=='6' || car=='7' || car=='8' || car=='9' {
//            
//        }
//    }
//    0.0
//
//}

//fn find_number(car_chain:&str, index:usize) -> f64{ 
//    if index < car_chain.len()-1 {
//        let mut thing: &str= &car_chain[index..];
//        for i in 0..thing.len() {
//            if thing.as_bytes()[i]==b'+' || thing.as_bytes()[i]==b'-' || thing.as_bytes()[i]==b'x' || thing.as_bytes()[i]==b'/' {
//                let new_thing = &thing[..i];
//                break
//            }
//        }
//        return match new_thing.parse::<f64>() {
//            Ok(num) => num,
//            Err(_) => 0.0
//        }, find_number(thing, new_thing.len())
//    }
//}